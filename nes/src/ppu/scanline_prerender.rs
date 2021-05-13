use super::{Pinout, Context};
use super::background::Background;
use super::sprites::Sprites;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

fn scanline_prerender_tick(ppu: &mut Context, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {

    match ppu.hpos {
        0 => {
            pinout = prerender_idle_cycle(ppu, mapper, pinout);
        },
        // tile data fetched
        1 => {
            ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, false);
            pinout = open_tile_index(ppu, mapper, pinout);
            ppu.hpos += 1;
        }
        2 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        3 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        4 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        5 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        6 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        7 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        8 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        9 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        10 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        11 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        12 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        13 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        14 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        15 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        16 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        17 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        18 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        19 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        20 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        21 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        22 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        23 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        24 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        25 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        26 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        27 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        28 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        29 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        30 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        31 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        32 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        33 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        34 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        35 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        36 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        37 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        38 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        39 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        40 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        41 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        42 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        43 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        44 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        45 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        46 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        47 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        48 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        49 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        50 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        51 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        52 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        53 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        54 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        55 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        56 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        57 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        58 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        59 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        60 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        61 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        62 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        63 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        64 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        65 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        66 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        67 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        68 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        69 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        70 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        71 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        72 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        73 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        74 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        75 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        76 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        77 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        78 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        79 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        80 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        81 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        82 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        83 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        84 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        85 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        86 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        87 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        88 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        89 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        90 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        91 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        92 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        93 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        94 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        95 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        96 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        97 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        98 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        99 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        100 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        101 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        102 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        103 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        104 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        105 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        106 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        107 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        108 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        109 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        110 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        111 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        112 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        113 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        114 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        115 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        116 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        117 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        118 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        119 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        120 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        121 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        122 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        123 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        124 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        125 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        126 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        127 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        128 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        129 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        130 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        131 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        132 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        133 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        134 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        135 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        136 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        137 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        138 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        139 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        140 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        141 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        142 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        143 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        144 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        145 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        146 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        147 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        148 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        149 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        150 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        151 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        152 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        153 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        154 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        155 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        156 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        157 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        158 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        159 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        160 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        161 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        162 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        163 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        164 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        165 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        166 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        167 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        168 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        169 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        170 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        171 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        172 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        173 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        174 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        175 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        176 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        177 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        178 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        179 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        180 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        181 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        182 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        183 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        184 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        185 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        186 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        187 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        188 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        189 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        190 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        191 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        192 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        193 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        194 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        195 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        196 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        197 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        198 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        199 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        200 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        201 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        202 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        203 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        204 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        205 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        206 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        207 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        208 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        209 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        210 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        211 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        212 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        213 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        214 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        215 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        216 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        217 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        218 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        219 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        220 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        221 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        222 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        223 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        224 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        225 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        226 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        227 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        228 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        229 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        230 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        231 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        232 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        233 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        234 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        235 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        236 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        237 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        238 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        239 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        240 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        241 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        242 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        243 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        244 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        245 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        246 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        247 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        248 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        249 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        250 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        251 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        252 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        253 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        254 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        255 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        256 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
            ppu.addr_reg.y_increment();
        }
        // sprite tile data fetched, garbage nt and attr fetched
        257 => {
            pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1;
            sp.clear_oam_addr();
            // update V horizontal bits
            ppu.addr_reg.update_x_scroll();
            ppu.hpos += 1;
        }
        258 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        259 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        260 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        261 => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        262 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        263 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        264 => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }

        265 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        266 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        267 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        268 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        269 => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        270 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        271 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        272 => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }

        273 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        274 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        275 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        276 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        277  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        278 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        279 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        // sprite data fetched, update V vertical bits
        280  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        281 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        282 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        283 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        284 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        285  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        286 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        287 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        288  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        289 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        290 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        291 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        292 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        293  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        294 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        295 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        296  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }

        297 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        298 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        299 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        300 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        301  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        302 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        303 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        304  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; ppu.addr_reg.update_vertical(); }
        // sprite data fetched
        305 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        306 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        307 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        308 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        309  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        310 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        311 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        312  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }

        313 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        314 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        315 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        316 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        317  => { pinout = open_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        318 => { pinout = read_sprite_pattern0(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        319 => { pinout = open_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        320  => { pinout = read_sprite_pattern1(ppu, sp, mapper, pinout); ppu.hpos += 1; }
        // tile data fetched for next scanline
        321 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        322 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        323 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        324 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        325 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        326 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        327 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        328 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        329 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        330 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        331 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        332 => { pinout = read_background_attribute(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        333 => { pinout = open_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        334 => { pinout = read_background_pattern0(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        335 => { pinout = open_background_pattern1(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        336 => {
            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        // garbage nt fetches
        337 => { pinout = open_tile_index(ppu, mapper, pinout); ppu.hpos += 1; }
        338 => { pinout = read_tile_index(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        339 => { pinout = open_background_attribute(ppu, mapper, pinout); ppu.hpos += 1; }
        340 => { 
            pinout = read_background_attribute(ppu, bg, mapper, pinout); 
            ppu.vpos = 0;
            // on odd frames this cycle is skipped , simulate by skipping the next render idle cycle
            ppu.hpos = if ppu.odd_frame { 1 } else { 0 };
        }
        _ => { panic!("prerender visible out of bounds"); }
    }

    pinout
}