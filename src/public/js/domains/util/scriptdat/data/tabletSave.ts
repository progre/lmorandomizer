import { LMWorld } from './Script';
import LMObject from './LMObject';
import addObject from './addObject';
import { romNumbers } from '../../../model/randomizer/items';
import F from './Flags';

export default function tabletSave(
    worlds: ReadonlyArray<LMWorld>,
    easyMode: boolean,
): ReadonlyArray<LMWorld> {
    const guisaves = [
        new LMObject(14, 14336, 40960, 200, -1, 185, 0, []),                                  // xelpud
        new LMObject(157, 12288, 36864, 4, 4, 10000, F.WARP_GUI, []),                           // stoparea -- activate Flags.WARP_GUI
        new LMObject(40, F.WARP_GUI, F.DO_WARP, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, F.WARP_SPR, []), // turn off other warp flags
        new LMObject(40, F.WARP_GUI, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_GUI, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_GUI, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const mausaves = [
        new LMObject(14, 16384, 32768, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 14336, 28672, 4, 4, 10000, F.WARP_MAU, []), // stoparea -- activate Flags.WARP_MAU
        new LMObject(40, F.WARP_MAU, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_SUN, F.WARP_SPR, []), // turn off other warp flags
        new LMObject(40, F.WARP_MAU, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_MAU, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_MAU, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const sunsaves = [
        new LMObject(14, 57344, 16384, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 55296, 12288, 4, 4, 10000, F.WARP_SUN, []), // stoparea -- activate Flags.WARP_SUN
        new LMObject(40, F.WARP_SUN, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SPR, []), // turn off other warp flags
        new LMObject(40, F.WARP_SUN, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_SUN, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_SUN, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const sprsaves = [
        new LMObject(14, 14336, 16384, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 12288, 12288, 4, 4, 10000, F.WARP_SPR, []), // stoparea -- activate Flags.WARP_SPR
        new LMObject(40, F.WARP_SPR, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_SPR, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_SPR, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const infsaves = [
        new LMObject(14, 30720, 8192, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 28672, 4096, 4, 4, 10000, F.WARP_INF, []), // stoparea -- activate Flags.WARP_INF
        new LMObject(40, F.WARP_INF, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_INF, F.WARP_SPR, F.WARP_EXT, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_INF, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_INF, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const extsaves = [
        new LMObject(14, 4096, 16384, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 2048, 12288, 4, 4, 10000, F.WARP_EXT, []), // stoparea -- activate Flags.WARP_EXT
        new LMObject(40, F.WARP_EXT, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_EXT, F.WARP_SPR, F.WARP_INF, F.WARP_TLF, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_EXT, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_EXT, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const tlfsaves = [
        new LMObject(14, 10240, 40960, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 8192, 36864, 4, 4, 10000, F.WARP_TLF, []), // stoparea -- activate Flags.WARP_TLF
        new LMObject(40, F.WARP_TLF, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_TLF, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_END, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_TLF, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_TLF, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const endsaves = [
        new LMObject(14, 45056, 8192, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 43008, 4096, 4, 4, 10000, F.WARP_END, []), // stoparea -- activate Flags.WARP_END
        new LMObject(40, F.WARP_END, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_END, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_MOM, []),     // ibid
        new LMObject(40, F.WARP_END, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_END, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const momsaves = [
        new LMObject(14, 43008, 8192, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 40960, 4096, 4, 4, 10000, F.WARP_MOM, []), // stoparea -- activate Flags.WARP_MOM
        new LMObject(40, F.WARP_MOM, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_MOM, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_MOM, F.WARP_CON, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_MOM, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const consaves = [
        new LMObject(14, 28672, 8192, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 26624, 4096, 4, 4, 10000, F.WARP_CON, []), // stoparea -- activate Flags.WARP_CON
        new LMObject(40, F.WARP_CON, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_CON, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_CON, F.WARP_MOM, F.WARP_GRA, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_CON, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const grasaves = [
        new LMObject(14, 16384, 16384, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 14336, 12288, 4, 4, 10000, F.WARP_GRA, []), // stoparea -- activate Flags.WARP_GRA
        new LMObject(40, F.WARP_GRA, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_GRA, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_GRA, F.WARP_MOM, F.WARP_CON, F.WARP_MOO, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_GRA, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const moonsaves = [
        new LMObject(14, 22528, 24576, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 20480, 20480, 4, 4, 10000, F.WARP_MOO, []), // stoparea -- activate Flags.WARP_MOO
        new LMObject(40, F.WARP_MOO, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_MOO, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_MOO, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_GOD, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_MOO, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const godsaves = [
        new LMObject(14, 30720, 8192, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 28672, 4096, 4, 4, 10000, F.WARP_GOD, []), // stoparea -- activate Flags.WARP_GOD
        new LMObject(40, F.WARP_GOD, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_GOD, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_GOD, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_MOO, F.WARP_RUI, []),     // ibid
        new LMObject(40, F.WARP_GOD, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const ruisaves = [
        new LMObject(14, 30720, 40960, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 28672, 36864, 4, 4, 10000, F.WARP_RUI, []), // stoparea -- activate Flags.WARP_RUI
        new LMObject(40, F.WARP_RUI, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_RUI, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_RUI, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_MOO, F.WARP_GOD, []),     // ibid
        new LMObject(40, F.WARP_RUI, F.WARP_BIR, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const birsaves = [
        new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 253952, 36864, 4, 4, 10000, F.WARP_BIR, []), // stoparea -- activate Flags.WARP_BIR
        new LMObject(40, F.WARP_BIR, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_BIR, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_BIR, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_MOO, F.WARP_GOD, []),     // ibid
        new LMObject(40, F.WARP_BIR, F.WARP_RUI, F.WARP_TLB, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const tlbsaves = [
        new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 253952, 36864, 4, 4, 10000, F.WARP_TLB, []), // stoparea -- activate Flags.WARP_TLB
        new LMObject(40, F.WARP_TLB, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_TLB, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_TLB, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_MOO, F.WARP_GOD, []),     // ibid
        new LMObject(40, F.WARP_TLB, F.WARP_RUI, F.WARP_BIR, F.WARP_DIM, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const dimsaves = [
        new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
        new LMObject(157, 253952, 36864, 4, 4, 10000, F.WARP_DIM, []), // stoparea -- activate Flags.WARP_TLB
        new LMObject(40, F.WARP_DIM, F.DO_WARP, F.WARP_GUI, F.WARP_SUR, F.WARP_MAU, F.WARP_SUN, []), // turn off other warp flags
        new LMObject(40, F.WARP_DIM, F.WARP_SPR, F.WARP_INF, F.WARP_EXT, F.WARP_TLF, F.WARP_END, []),     // ibid
        new LMObject(40, F.WARP_DIM, F.WARP_MOM, F.WARP_GRA, F.WARP_CON, F.WARP_MOO, F.WARP_GOD, []),     // ibid
        new LMObject(40, F.WARP_DIM, F.WARP_RUI, F.WARP_BIR, F.WARP_TLB, -1, -1, []),                 // ibid
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
    ];
    const surfsaves = [
        new LMObject(87, 26624, 16384, 18, 1, 15, 4,
            [{ number: F.DO_WARP, value: true },
                { number: F.WARP_SUR, value: false },
            ]
        ), // warp to center of (unused) screen
        new LMObject(40, F.DO_WARP, F.DO_WARP, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
        new LMObject(157, 16384, 12288, 4, 4, 10000, F.WARP_SUR, []), // set Flags.WARP_SUR when near tent
    ];
    const gotwarps = [
        new LMObject(87, 30720, 16384, 1, 7, 13, 5, []), // warp to surface is lower. player should not reach this.
        new LMObject(87, 30720, 14336, 0, 6, 7, 19, [{ number: F.WARP_GUI, value: true }]), // warp to guidance
        new LMObject(87, 30720, 14336, 2, 8, 8, 15, [{ number: F.WARP_MAU, value: true }]), // warp to mausoleum
        new LMObject(87, 30720, 14336, 3, 2, 28, 7, [{ number: F.WARP_SUN, value: true }]), // warp to sun
        new LMObject(87, 30720, 14336, 4, 7, 7, 7, [{ number: F.WARP_SPR, value: true }]), // warp to spring
        new LMObject(87, 30720, 14336, 5, 14, 15, 3, [{ number: F.WARP_INF, value: true }]), // warp to inferno
        new LMObject(87, 30720, 14336, 6, 19, 2, 7, [{ number: F.WARP_EXT, value: true }]), // warp to extinction
        new LMObject(87, 30720, 14336, 9, 0, 5, 19, [{ number: F.WARP_TLF, value: true }]), // warp to twin labyrinths (front)
        new LMObject(87, 30720, 14336, 7, 0, 22, 3, [{ number: F.WARP_END, value: true }]), // warp to endless corridor
        new LMObject(87, 30720, 14336, 8, 17, 21, 30, [{ number: F.WARP_MOM, value: true }]), // warp to shrine of the mother
        new LMObject(87, 30720, 14336, 11, 4, 14, 3, [{ number: F.WARP_CON, value: true }]), //warp to confusion gate
        new LMObject(87, 30720, 14336, 12, 7, 8, 7, [{ number: F.WARP_GRA, value: true }]), // warp to graveyard
        new LMObject(87, 30720, 14336, 14, 4, 11, 11, [{ number: F.WARP_MOO, value: true }]), // warp to moonlight
        new LMObject(87, 30720, 14336, 13, 16, 15, 3, [{ number: F.WARP_GOD, value: true }]), // warp to goddess
        new LMObject(87, 30720, 14336, 15, 4, 15, 19, [{ number: F.WARP_RUI, value: true }]), // warp to ruin
        new LMObject(87, 30720, 14336, 16, 3, 29, 19, [{ number: F.WARP_BIR, value: true }]), // warp to birth
        new LMObject(87, 30720, 14336, 10, 3, 25, 19, [{ number: F.WARP_TLB, value: true }]), // warp to twin labyrinths (back)
        new LMObject(87, 30720, 14336, 17, 10, 9, 11, [{ number: F.WARP_DIM, value: true }]), // warp to dimensional corridor
    ];

    if (!easyMode) {
        
        surfsaves.push(
            new LMObject(22, 26624, 10240, 2, 2, F.EASY_RESPAWN, -1, []),
            new LMObject(1, 26624, 14336, F.EASY_RESPAWN, 100 + romNumbers.gameMaster, F.EASY_SAVE, -1,
                [{ number: 99999, value: true }, { number: F.EASY_SAVE, value: false }], )
        );
    }
    worlds = addObject(worlds, 0, 2, 1, guisaves);
    worlds = addObject(worlds, 1, 3, 1, surfsaves);
    worlds = addObject(worlds, 2, 0, 2, mausaves);
    worlds = addObject(worlds, 3, 2, 0, sunsaves);
    worlds = addObject(worlds, 4, 1, 3, sprsaves);
    worlds = addObject(worlds, 5, 2, 3, infsaves);
    worlds = addObject(worlds, 6, 3, 4, extsaves);
    worlds = addObject(worlds, 7, 0, 0, endsaves);
    worlds = addObject(worlds, 8, 1, 4, momsaves);
    worlds = addObject(worlds, 9, 0, 0, tlfsaves);
    worlds = addObject(worlds, 10, 3, 0, tlbsaves);
    worlds = addObject(worlds, 11, 0, 1, consaves);
    worlds = addObject(worlds, 12, 3, 1, grasaves);
    worlds = addObject(worlds, 13, 0, 4, godsaves);
    worlds = addObject(worlds, 14, 0, 1, moonsaves); // not a typo
    worlds = addObject(worlds, 15, 0, 1, ruisaves);
    worlds = addObject(worlds, 16, 3, 0, birsaves);
    worlds = addObject(worlds, 17, 2, 2, dimsaves);

    worlds = addObject(worlds, 18, 1, 0, gotwarps);
    return worlds;
}