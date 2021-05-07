use super::{Pinout, Context};
use super::background::Background;
use super::sprites::Sprites;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

fn scanline_prerender(ppu: &mut Context, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) {

    match ppu.scanline_dot {
        0 => {
            ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            // Read first bytes of secondary OAM
            pinout = render_idle_cycle(ppu, mapper, pinout);
        },
        // tile data fetched
        1 => {
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, false);
            pinout = open_tile_index(ppu, mapper, pinout);
        }
        2 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        3 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        4 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        5 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        6 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        7 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        8 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        9 => { pinout = open_tile_index(ppu, mapper, pinout); }
        10 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        11 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        12 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        13 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        14 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        15 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        16 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        17 => { pinout = open_tile_index(ppu, mapper, pinout); }
        18 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        19 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        20 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        21 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        22 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        23 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        24 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        25 => { pinout = open_tile_index(ppu, mapper, pinout); }
        26 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        27 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        28 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        29 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        30 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        31 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        32 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        33 => { pinout = open_tile_index(ppu, mapper, pinout); }
        34 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        35 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        36 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        37 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        38 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        39 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        40 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        41 => { pinout = open_tile_index(ppu, mapper, pinout); }
        42 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        43 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        44 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        45 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        46 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        47 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        48 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        49 => { pinout = open_tile_index(ppu, mapper, pinout); }
        50 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        51 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        52 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        53 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        54 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        55 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        56 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        57 => { pinout = open_tile_index(ppu, mapper, pinout); }
        58 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        59 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        60 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        61 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        62 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        63 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        64 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        65 => { pinout = open_tile_index(ppu, mapper, pinout); }
        66 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        67 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        68 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        69 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        70 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        71 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        72 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        73 => { pinout = open_tile_index(ppu, mapper, pinout); }
        74 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        75 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        76 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        77 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        78 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        79 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        80 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        81 => { pinout = open_tile_index(ppu, mapper, pinout); }
        82 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        83 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        84 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        85 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        86 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        87 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        88 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        89 => { pinout = open_tile_index(ppu, mapper, pinout); }
        90 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        91 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        92 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        93 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        94 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        95 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        96 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        97 => { pinout = open_tile_index(ppu, mapper, pinout); }
        98 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        99 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        100 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        101 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        102 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        103 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        104 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        105 => { pinout = open_tile_index(ppu, mapper, pinout); }
        106 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        107 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        108 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        109 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        110 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        111 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        112 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        113 => { pinout = open_tile_index(ppu, mapper, pinout); }
        114 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        115 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        116 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        117 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        118 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        119 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        120 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        121 => { pinout = open_tile_index(ppu, mapper, pinout); }
        122 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        123 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        124 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        125 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        126 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        127 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        128 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        129 => { pinout = open_tile_index(ppu, mapper, pinout); }
        130 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        131 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        132 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        133 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        134 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        135 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        136 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        137 => { pinout = open_tile_index(ppu, mapper, pinout); }
        138 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        139 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        140 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        141 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        142 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        143 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        144 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        145 => { pinout = open_tile_index(ppu, mapper, pinout); }
        146 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        147 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        148 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        149 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        150 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        151 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        152 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        153 => { pinout = open_tile_index(ppu, mapper, pinout); }
        154 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        155 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        156 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        157 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        158 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        159 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        160 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        161 => { pinout = open_tile_index(ppu, mapper, pinout); }
        162 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        163 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        164 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        165 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        166 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        167 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        168 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        169 => { pinout = open_tile_index(ppu, mapper, pinout); }
        170 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        171 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        172 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        173 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        174 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        175 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        176 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        177 => { pinout = open_tile_index(ppu, mapper, pinout); }
        178 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        179 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        180 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        181 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        182 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        183 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        184 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        185 => { pinout = open_tile_index(ppu, mapper, pinout); }
        186 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        187 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        188 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        189 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        190 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        191 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        192 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        193 => { pinout = open_tile_index(ppu, mapper, pinout); }
        194 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        195 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        196 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        197 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        198 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        199 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        200 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        201 => { pinout = open_tile_index(ppu, mapper, pinout); }
        202 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        203 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        204 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        205 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        206 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        207 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        208 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        209 => { pinout = open_tile_index(ppu, mapper, pinout); }
        210 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        211 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        212 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        213 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        214 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        215 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        216 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        217 => { pinout = open_tile_index(ppu, mapper, pinout); }
        218 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        219 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        220 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        221 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        222 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        223 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        224 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        225 => { pinout = open_tile_index(ppu, mapper, pinout); }
        226 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        227 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        228 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        229 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        230 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        231 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        232 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        233 => { pinout = open_tile_index(ppu, mapper, pinout); }
        234 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        235 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        236 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        237 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        238 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        239 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        240 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        241 => { pinout = open_tile_index(ppu, mapper, pinout); }
        242 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        243 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        244 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        245 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        246 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        247 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        248 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        249 => { pinout = open_tile_index(ppu, mapper, pinout); }
        250 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        251 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        252 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        253 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        254 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        255 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        256 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.addr_reg.y_increment();
        }
        // sprite tile data fetched, garbage nt and attr fetched
        257 => {
            pinout = open_tile_index(ppu, mapper, pinout);
            sp.clear_oam_addr();
            // update V horizontal bits
            ppu.addr_reg.update_x_scroll();
        }
        258 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        259 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        260 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        261 => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); }
        262 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); }
        263 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); }
        264 => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); }
        265 => { pinout = open_tile_index(ppu, mapper, pinout); }
        266 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        267 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        268 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        269 => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); }
        270 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); }
        271 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); }
        272 => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); }
        273 => { pinout = open_tile_index(ppu, mapper, pinout); }
        274 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        275 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        276 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        277  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); }
        278 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); }
        279 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); }
        // sprite data fetched, update V vertical bits
        280  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        281 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        282 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        283 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        284 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        285  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        286 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        287 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        288  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        289 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        290 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        291 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        292 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        293  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        294 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        295 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        296  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        297 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        298 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        299 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.addr_reg.update_vertical(); }
        300 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.addr_reg.update_vertical(); }
        301  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        302 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        303 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        304  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.addr_reg.update_vertical(); }
        // sprite data fetched
        305 => { pinout = open_tile_index(ppu, mapper, pinout); }
        306 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        307 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        308 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        309  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); }
        310 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); }
        311 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); }
        312  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); }
        313 => { pinout = open_tile_index(ppu, mapper, pinout); }
        314 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        315 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        316 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        317  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); }
        318 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); }
        319 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); }
        320  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); }
        // tile data fetched for next scanline
        321 => { pinout = open_tile_index(ppu, mapper, pinout); }
        322 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        323 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        324 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        325 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        326 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        327 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        328 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        329 => { pinout = open_tile_index(ppu, mapper, pinout); }
        330 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        331 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        332 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        333 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); }
        334 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); }
        335 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); }
        336 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
        }
        // garbage nt fetches
        337 => { pinout = open_tile_index(ppu, mapper, pinout); }
        338 => { pinout = read_tile_index(ppu, bg, mapper, pinout); }
        339 => { pinout = open_background_attribute(ppu, mapper, pinout); }
        340 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); }
        _ => { panic!("prerender visible out of bounds"); }
    }
}