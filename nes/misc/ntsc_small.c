#include <cmath>
#include <vector>
#include <algorithm>
#include <cstdio>

namespace
{
    // This function generates a testcard resembling Philips PM5544 testcard, but with NES colors.
    // Inputs:  x = x coordinate, y = y coordinate
    // Output:  NES color index (0x00-0x3F = main palette color, plus 3 bits of color de-emphasis).
    /* The only purpose of this function is to generate a sample NES screen without any special processing. */
    unsigned TestCard(int x,int y)
    {
        const float aspect = 8./7;
        // Define the geometry
        const int width   = 256,     height  = 240;
        const int xcenter = width/2, ycenter = height/2;
        // Define colors
        const int black = 0x1D, white = 0x30;
        // Define background grid
        int xgrid = ((x+8)>>4)-1, xgridsub = (x+8)&15, ygrid = ((y+8)>>4)-1, ygridsub = (y+8)&15;
        int c = (xgridsub == 0 || ygridsub == 0) ? white // white grid
            : ((ygrid < 0 || ygrid >= 14)
               ? (xgrid & 1) ? white : black // white or black checkerboard at top & bottom
               : 0x00); // gray otherwise

        // Define some lobes
        if(y > 24 && ygrid < 13)
        {
            static const char lobe[] = {0x15/*90*/,0x1A/*250*/, 0x17/*326*/,0x1C/*146*/, 0x14/*0*/,0x18/*180*/};
            bool nook = (ygrid < 3 || (ygrid >= 11 && (ygrid>11 || ygridsub>0)));
            if(xgrid >= 0 && xgrid <= 1
            && (xgrid == 0 ? xgridsub > 0 : nook)
            ) c = lobe[(y < ycenter) + 2*(xgrid==1)];

            if(xgrid >= 13 && xgrid <= 14
            && (xgrid == 14 ? xgridsub > 0 || nook : (nook && xgridsub > 0))
            ) c = lobe[(y < ycenter) + 4 - 2*(xgrid==13)];
        }

        // Define circle
        const int xdist = std::abs(x-xcenter);
        const int ydist = std::abs(y-ycenter);
        float xdista = xdist*aspect;
        if(std::sqrt(xdista*xdista + ydist*ydist) >= 6.7*16) return c;

        c = white; // white for circle
        if((xdist < 8 && ydist < 24 && y < 7*16) || ydist <= 8)
        {
            if(xgridsub != 0 && ygridsub != 0) c = black;
            if(xdist == 0) c = white; // white vertical line for center
            return c;
        }
        // Define some exceptional content
        switch(ygrid)
        {
            case 1: if( xdist < width*0.12 )
                c = 0x0E + 0x10 * ((x>>3) & 3); // Use four different blacks for the station ID background
                break;
            case 2: if( xdist > width*0.18 || x == (int)(width*0.35)) c = black; break;
            case 11: if( xdist <= width*0.18 && x != (int)(width*0.35))
                c = 0x0F + 0x10 * ((x>>3) & 3); // Use four different blacks for the station ID background
                break;
            case 3: c = (x+5)%30 < 15 ? 0x10 : black; break; // gray or black
            case 4: case 5: case 6:
            {
                int freq = 3 - (int)(x / 64);
                int sub = (y - 8 - 4*16);
                c = (((x >> freq) ^ (int)(sub / 20)) & 1) ? white : black;
                c |= 0x40 * (int)(sub / 6); // Add vertically changing color de-emphasis
                break;
            }
            case 7: case 8: case 9:
            {
                int sub = ((y - 8*16) >> 3);
                float firstx = xcenter - 96*aspect;
                float lastx    = xcenter + 96*aspect;
                float xpos = (x-firstx) / (lastx-firstx);
                if(sub < 4) c = ((int)(14 * xpos)&15) + 16 * (sub&3);
                break;
            }
            case 10:
            {
                static const char gradient[] = {0x0D,0x1D,0x2D,0x00, 0x10,0x3D,0x20,0x30};
                float firstx = xcenter - 67*aspect;
                float lastx    = xcenter + 67*aspect;
                float xpos = (x-firstx) / (lastx-firstx);
                c = gradient[(int)(0.5 + (sizeof(gradient)-1) * xpos)];
                break;
            }
            case 12: case 13: c = xdist < 8 ? 0x16 : 0x28; break;
        }
        return c & 0x1FF;
    }
    /* End testcard generator. */

    // PNGencoder: A minimal PNG encoder for demonstration purposes.
    class PNGencoder
    {
        std::vector<unsigned char> output;
    private:
        static void PutWord(unsigned char* target, unsigned dword, bool msb_first)
        {
            for(int p=0; p<4; ++p) target[p] = dword >> (msb_first ? 24-p*8 : p*8);
        }
        static unsigned adler32(const unsigned char* data, unsigned size, unsigned res=1)
        {
            for(unsigned s1,a=0; a<size; ++a)
                s1 = ((res & 0xFFFF) + (data[a] & 0xFF)) % 65521,
                    res = s1 + ((((res>>16) + s1) % 65521) << 16);
            return res;
        }
        static unsigned crc32(const unsigned char* data, unsigned size, unsigned res=0)
        {
            for(unsigned tmp,n,a=0; a<size; ++a, res = ~((~res>>8) ^ tmp))
                for(tmp = (~res ^ data[a]) & 0xFF, n=0; n<8; ++n)
                    tmp = (tmp>>1) ^ ((tmp&1) ? 0xEDB88320u : 0);
            return res;
        }
        void Deflate(const unsigned char* source, unsigned srcsize)
        {
            /* A bare-bones deflate-compressor (as in gzip) */
            int algo=8, windowbits=8 /* 8 to 15 allowed */, flevel=0 /* 0 to 3 */;
            /* Put RFC1950 header: */
            unsigned h0 = algo + (windowbits-8)*16, h1 = flevel*64 + 31-((256*h0)%31);
            output.push_back(h0);
            output.push_back(h1); /* checksum and compression level */
            /* Compress data using a lossless algorithm (RFC1951): */
            for(unsigned begin=0; ; )
            {
                unsigned eat = std::min(65535u, srcsize-begin);

                std::size_t o = output.size(); output.resize(o+5);
                output[o+0] = (begin+eat) >= srcsize; /* bfinal bit and btype: 0=uncompressed */
                PutWord(&output[o+1], eat|(~eat<<16), false);
                output.insert(output.end(), source+begin, source+begin+eat);
                begin += eat;
                if(begin >= srcsize) break;
            }
            /* After compressed data, put the checksum (adler-32): */
            std::size_t o = output.size(); output.resize(o+4);
            PutWord(&output[o], adler32(source, srcsize), true);
        }
    public:
        void EncodeImage(unsigned width,unsigned height, const unsigned char* rgbdata)
        {
            std::size_t o;
            #define BeginChunk(n_reserve_extra) o = output.size(); output.resize(o+8+n_reserve_extra)
            #define EndChunk(type) do { \
                unsigned ncopy=output.size()-(o+8); \
                static const char t[4+1] = type; \
                output.resize(o+8+ncopy+4); \
                PutWord(&output[o+0], ncopy, true); /* chunk length */ \
                std::copy(t+0, t+4, &output[o+4]);  /* chunk type */   \
                /* in the chunk, put crc32 of the type and data (below) */ \
                PutWord(&output[o+8+ncopy], crc32(&output[o+4], 4+ncopy), true); \
            } while(0)
            /* Put PNG header (always const) */
            static const char header[8+1] = "\x89PNG\15\12\x1A\12";
            output.insert(output.end(), header, header+8);
            /* Put IHDR chunk */
            BeginChunk(13);
              PutWord(&output[o+8+0], width,  true); /* Put image width    */
              PutWord(&output[o+8+4], height, true); /* and height in IHDR */
              PutWord(&output[o+8+8], 0x08020000, true);
              /* Meaning of above: 8-bit,rgb-triple,deflate,std filters,no interlacing */
            EndChunk("IHDR");
            /* Put IDAT chunk */
            BeginChunk(0);
              std::vector<unsigned char> idat;
              for(unsigned y=0; y<height; ++y)
              {
                  idat.push_back(0x00); // filter type for this scanline
                  idat.insert(idat.end(), rgbdata+y*width*3, rgbdata+y*width*3+width*3);
              }
              Deflate(&idat[0], idat.size());
            EndChunk("IDAT");
            /* Put IEND chunk */
            BeginChunk(0);
            EndChunk("IEND");
            #undef BeginChunk
            #undef EndChunk
        }
        void SaveTo(const char* fn)
        {
            std::FILE* fp = std::fopen(fn, "wb");
            if(!fp) { std::perror(fn); return; }
            std::fwrite(&output[0], 1, output.size(), fp);
            std::fclose(fp);
        }
    };
    /* End PNG encoder. */

    // Define the NTSC voltage levels corresponding to each of the four different pixel colors.
    const float Voltages[4][2] = { {0.350f, 1.090f},
                                   {0.518f, 1.500f},
                                   {0.962f, 1.960f},
                                   {1.550f, 1.960f} };
}

/* MAIN PROGRAM */
int main(int argc, char** argv)
{
    if(argc != 4)
    {
        std::fprintf(stderr, "Usage: prog <width> <height> <output filename>\n");
        return 0;
    }
    unsigned output_width  = std::atoi(argv[1]);
    unsigned output_height = std::atoi(argv[2]);

    std::vector<unsigned char> rgbdata;
    rgbdata.resize( output_width*output_height*3 );

    // The NTSC signal decoder maintains a buffer of previous values.
    const unsigned SignalBufferWidth = 24, YtuningQuality = 12, IQtuningQuality = 0;
    float SignalHistory[SignalBufferWidth] = {0.f}, SumY, SumI, SumQ;
    unsigned ColorPhase  = 4; // Initial hue / phase value for this frame.
    unsigned DecodePhase = (SignalBufferWidth-0-ColorPhase) % SignalBufferWidth;
    SumY = -6.f; // <- I'm not sure where this comes from. Maybe floating point problems. Should be zero.
    SumI = SumQ = 0.f;

    unsigned cycles = 12; // Initial cycles value for this frame.

    // Generate picture. 240 scanlines.
    for(unsigned y=0; y<240; ++y)
    {
        /* For horizontal position in output pixture: */
        unsigned pixelcarry = 0, hpos = 0;
        /* For vertical position in output picture: */
        unsigned vpos = y*output_height/240, n_lines = (y+1)*output_height/240 - vpos;

        // Render 256 columns.
        for(unsigned x=0; x<256; ++x)
        {
            // Retrieve current pixel color from the PPU.
            unsigned color = TestCard(x,y);

            // Divide the NES palette color into "brightness", "colour", and "de-emphasis".
            unsigned brightness = (color >> 4) & 3, index = (color & 0xF);
            // If the color is xE or xF, treat it as if it were 1D.
            if(index >= 0x0E) { brightness = 1, index = 0xD; }

            // Generate 8 samples of NTSC signal corresponding to this color.

            // Determine which voltage levels to use.
            float low = Voltages[brightness][0], high = Voltages[brightness][1];
            // If the color is monochrome, use either the low or the high and not both.
            if(index == 0x0D) high = low; else if(index == 0x00) low = high;
            // Determine initial phase.
            unsigned phase = 1 << ((cycles - index) % 12);

            // Determine which phases count for de-emphasis
            unsigned deemp = 0;
            if(color & 0x40) deemp |= (0x3F03F / 0x001);
            if(color & 0x80) deemp |= (0x3F03F / 0x100);
            if(color &0x100) deemp |= (0x3F03F / 0x010);

            // Now generate the 8 samples.
            for(unsigned n=0; n<8; ++n)
            {
                float sample = high;

                // The current phase cycles between 12 different phases.
                phase <<= 1;
                if(phase >= (1 << 12))
                {
                    phase = 1; // 12 phases covered, resets back to beginning
                }
                else
                {
                    // During 6 phases, high is generated; during the other 6, low is generated.
                    if(phase >= (1 << 6))
                        sample = low;
                }

                // If this cycle is to be de-emphasized, attenuate the sample by about 25 %.
                if(phase & deemp)
                {
                    sample *= 0.746f;
                }
                ++cycles;

                // Done generating the NTSC signal sample ("sample").

                // Begin decoding the NTSC signal!
                if(!DecodePhase--) DecodePhase = SignalBufferWidth-1;
                float oldspot1 = SignalHistory[(DecodePhase + SignalBufferWidth - YtuningQuality) % SignalBufferWidth];
                float oldspot2 = SignalHistory[(DecodePhase + SignalBufferWidth - IQtuningQuality) % SignalBufferWidth];
                SignalHistory[DecodePhase] = sample;
                float spot1 = sample - oldspot1;
                float spot2 = sample - oldspot2;
                // Note that this calculation completely cancels out any DC component in the input signal,
                // only keeping track of the magnitudes of deltas between signal level changes.
                SumY += spot1;
                SumI += spot2 * std::cos( (DecodePhase+3 - 1.9f) * 3.141592653f/4 * 8.f / 12.f );
                SumQ += spot2 * std::cos( (DecodePhase+0 - 1.9f) * 3.141592653f/4 * 8.f / 12.f );
                // Done decoding NTSC signal: It has been decomposed into Y, I and Q.

                // Note that any smart implementation would precalculate the std::cos() values
                // into a small compile-time constant array and use that array here instead of
                // calling std::cos() over and over again.
                // Making that change into the program is left as an exercise to the reader.

                // Check if we need to render a pixel now.
                pixelcarry += output_width;
                while(pixelcarry >= 256*8)
                {
                    // Yes, render pixel
                    pixelcarry -= 256*8;

                    // Convert YIQ into RGB:
                    float r = (SumY + SumI * 0.946882f + SumQ * 0.623557f) / SignalBufferWidth;;
                    float g = (SumY + SumI *-0.274788f + SumQ *-0.635691f) / SignalBufferWidth;;
                    float b = (SumY + SumI *-1.108545f + SumQ * 1.709007f) / SignalBufferWidth;;

                    // Convert the float RGB into RGB24:
                    int rr = r*256; if(rr<0) rr=0; else if(rr>255) rr=255;
                    int gg = g*256; if(gg<0) gg=0; else if(gg>255) gg=255;
                    int bb = b*256; if(bb<0) bb=0; else if(bb>255) bb=255;

                    // Save the RGB into the frame buffer. n_lines = number of times to duplicate that pixel vertically.
                    for(unsigned l=0; l<n_lines; ++l)
                    {
                        rgbdata[ ((vpos+l) * output_width + hpos) * 3 + 0 ] = rr;
                        rgbdata[ ((vpos+l) * output_width + hpos) * 3 + 1 ] = gg;
                        rgbdata[ ((vpos+l) * output_width + hpos) * 3 + 2 ] = bb;
                    }
                    ++hpos;
                }
            }
        }
    }

    PNGencoder enc;
    enc.EncodeImage(output_width, output_height, &rgbdata[0]);
    enc.SaveTo(argv[3]);
    return 0;
}