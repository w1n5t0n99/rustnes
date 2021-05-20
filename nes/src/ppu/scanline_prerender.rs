use super::Context;
use super::bus::Bus;
use super::background::Background;
use super::sprites::Sprites;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

pub fn scanline_prerender_tick(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper) {

    match ppu.hpos {
        0 => { prerender_idle_cycle(ppu, bus, mapper); ppu.hpos += 1; },
        // tile data fetched
        1 => {
            ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, false);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        2 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        3 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        4 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        5 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        6 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        7 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        8 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        9 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        10 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        11 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        12 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        13 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        14 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        15 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        16 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        17 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        18 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        19 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        20 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        21 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        22 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        23 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        24 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        25 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        26 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        27 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        28 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        29 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        30 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        31 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        32 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        33 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        34 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        35 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        36 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        37 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        38 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        39 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        40 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        41 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        42 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        43 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        44 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        45 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        46 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        47 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        48 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        49 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        50 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        51 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        52 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        53 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        54 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        55 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        56 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        57 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        58 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        59 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        60 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        61 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        62 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        63 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        64 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        65 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        66 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        67 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        68 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        69 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        70 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        71 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        72 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        73 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        74 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        75 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        76 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        77 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        78 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        79 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        80 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        81 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        82 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        83 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        84 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        85 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        86 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        87 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        88 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        89 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        90 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        91 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        92 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        93 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        94 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        95 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        96 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        97 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        98 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        99 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        100 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        101 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        102 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        103 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        104 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        105 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        106 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        107 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        108 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        109 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        110 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        111 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        112 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        113 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        114 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        115 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        116 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        117 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        118 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        119 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        120 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        121 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        122 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        123 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        124 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        125 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        126 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        127 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        128 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        129 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        130 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        131 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        132 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        133 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        134 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        135 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        136 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        137 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        138 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        139 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        140 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        141 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        142 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        143 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        144 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        145 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        146 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        147 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        148 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        149 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        150 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        151 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        152 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        153 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        154 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        155 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        156 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        157 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        158 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        159 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        160 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        161 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        162 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        163 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        164 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        165 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        166 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        167 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        168 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        169 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        170 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        171 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        172 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        173 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        174 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        175 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        176 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        177 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        178 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        179 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        180 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        181 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        182 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        183 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        184 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        185 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        186 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        187 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        188 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        189 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        190 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        191 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        192 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        193 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        194 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        195 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        196 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        197 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        198 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        199 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        200 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        201 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        202 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        203 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        204 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        205 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        206 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        207 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        208 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        209 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        210 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        211 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        212 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        213 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        214 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        215 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        216 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        217 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        218 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        219 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        220 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        221 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        222 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        223 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        224 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        225 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        226 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        227 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        228 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        229 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        230 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        231 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        232 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        233 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        234 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        235 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        236 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        237 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        238 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        239 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        240 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        241 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        242 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        243 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        244 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        245 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        246 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        247 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        248 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        249 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        250 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        251 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        252 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        253 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        254 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        255 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        256 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
            ppu.addr_reg.y_increment();
        }
        // sprite tile data fetched, garbage nt and attr fetched
        257 => {
            open_tile_index(ppu, bus, mapper); ppu.hpos += 1;
            sp.clear_oam_addr();
            // update V horizontal bits
            ppu.addr_reg.update_x_scroll();
            ppu.hpos += 1;
        }
        258 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        259 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        260 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        261 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        262 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        263 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        264 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }

        265 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        266 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        267 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        268 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        269 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        270 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        271 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        272 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }

        273 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        274 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        275 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        276 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        277  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        278 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        279 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        // sprite data fetched, update V vertical bits
        280  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        281 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        282 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        283 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        284 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        285  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        286 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        287 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        288  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        289 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        290 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        291 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        292 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        293  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        294 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        295 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        296  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        297 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        298 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        299 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        300 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        301  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        302 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        303 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        304  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        // sprite data fetched
        305 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        306 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        307 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        308 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        309  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        310 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        311 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        312  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }

        313 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        314 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        315 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        316 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        317  => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        318 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        319 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        320  => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        // tile data fetched for next scanline
        321 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        322 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        323 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        324 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        325 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        326 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        327 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        328 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        329 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        330 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        331 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        332 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        333 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        334 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        335 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        336 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        // garbage nt fetches
        337 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        338 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        339 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        340 => { 
            read_tile_index(ppu, bus, bg, mapper); 
            ppu.vpos = 0;
            // on odd frames this cycle is skipped , simulate by skipping the next render idle cycle
            ppu.hpos = if ppu.odd_frame { 1 } else { 0 };
        }
        _ => { panic!("prerender visible out of bounds"); }
    }
}

pub fn scanline_prerender_nonvisible_tick(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    match ppu.hpos {
        0 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        1 => {
            ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, false);
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        2..=339 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        340 => {
            nonrender_cycle(ppu, bus, mapper);
            // render idle cycle is not skipped if rendering disabled
            ppu.hpos = 0;
            ppu.vpos = 0;
        }
        _ => {
            panic!("PPU nonrender 0-340 out of bounds");
        }
    }
}