use std::collections::HashMap;
use std::collections::HashSet;
fn fill(
    m: &mut HashMap<(i32, i32), (i32, i32)>,
    (idx, top_left_x, top_left_y, width, heigth): (i32, i32, i32, i32, i32),
    nc: &mut HashSet<i32>,
) {
    for j in top_left_x..top_left_x + width {
        for i in top_left_y..top_left_y + heigth {
            let vv = m
                .entry((i, j))
                .and_modify(|(_, v)| *v += 1)
                .or_insert((idx, 1));
            if vv.1 > 1 {
                nc.remove(&idx);
                nc.remove(&vv.0);
            }
        }
    }
}

fn main() {
    let input = "#1 @ 55,885: 22x10
#2 @ 102,14: 23x14
#3 @ 539,327: 21x22
#4 @ 429,353: 14x25
#5 @ 232,934: 29x11
#6 @ 796,785: 17x18
#7 @ 508,96: 11x18
#8 @ 83,289: 28x23
#9 @ 291,46: 21x17
#10 @ 505,954: 23x15
#11 @ 934,606: 17x25
#12 @ 125,764: 19x16
#13 @ 699,475: 25x23
#14 @ 517,816: 13x12
#15 @ 983,477: 16x12
#16 @ 442,603: 18x24
#17 @ 330,620: 16x26
#18 @ 347,266: 25x29
#19 @ 679,465: 20x29
#20 @ 18,956: 17x18
#21 @ 204,804: 11x26
#22 @ 566,263: 24x22
#23 @ 771,152: 16x19
#24 @ 982,617: 15x28
#25 @ 932,20: 27x11
#26 @ 797,194: 11x29
#27 @ 596,666: 17x21
#28 @ 388,129: 29x10
#29 @ 268,255: 17x23
#30 @ 824,88: 15x24
#31 @ 408,60: 22x26
#32 @ 41,554: 20x13
#33 @ 902,136: 8x3
#34 @ 698,656: 17x13
#35 @ 270,904: 14x29
#36 @ 97,581: 10x20
#37 @ 72,669: 14x26
#38 @ 821,328: 11x17
#39 @ 884,825: 25x18
#40 @ 192,271: 17x17
#41 @ 196,928: 21x20
#42 @ 771,246: 18x23
#43 @ 91,680: 17x11
#44 @ 352,264: 20x28
#45 @ 403,640: 18x13
#46 @ 937,674: 28x14
#47 @ 790,602: 28x24
#48 @ 377,344: 17x15
#49 @ 24,833: 13x23
#50 @ 438,188: 22x25
#51 @ 935,828: 13x29
#52 @ 914,940: 18x17
#53 @ 233,347: 26x28
#54 @ 884,655: 21x14
#55 @ 219,643: 16x25
#56 @ 867,12: 18x16
#57 @ 885,640: 12x22
#58 @ 303,162: 10x14
#59 @ 852,140: 21x12
#60 @ 98,279: 19x15
#61 @ 73,894: 24x17
#62 @ 848,961: 23x14
#63 @ 344,372: 22x15
#64 @ 790,28: 4x8
#65 @ 956,18: 27x28
#66 @ 90,259: 27x20
#67 @ 514,570: 16x14
#68 @ 667,637: 17x15
#69 @ 853,403: 18x23
#70 @ 343,216: 14x21
#71 @ 63,978: 19x12
#72 @ 801,246: 16x28
#73 @ 304,968: 16x12
#74 @ 618,93: 20x21
#75 @ 885,791: 29x19
#76 @ 922,235: 22x24
#77 @ 688,486: 21x17
#78 @ 579,204: 21x29
#79 @ 177,745: 24x14
#80 @ 902,271: 24x12
#81 @ 508,404: 19x25
#82 @ 55,970: 10x25
#83 @ 519,537: 21x28
#84 @ 299,951: 10x27
#85 @ 950,702: 22x23
#86 @ 958,853: 13x14
#87 @ 608,467: 21x25
#88 @ 43,338: 27x17
#89 @ 367,425: 14x20
#90 @ 419,645: 11x18
#91 @ 728,274: 20x28
#92 @ 796,697: 13x16
#93 @ 607,654: 11x17
#94 @ 803,159: 24x17
#95 @ 953,199: 29x15
#96 @ 877,25: 19x18
#97 @ 967,475: 15x10
#98 @ 318,767: 18x11
#99 @ 55,899: 11x12
#100 @ 915,196: 15x20
#101 @ 821,144: 28x22
#102 @ 837,551: 20x17
#103 @ 894,858: 21x20
#104 @ 458,94: 20x20
#105 @ 854,256: 10x26
#106 @ 674,197: 17x26
#107 @ 570,327: 10x13
#108 @ 521,383: 13x15
#109 @ 406,509: 10x21
#110 @ 390,587: 20x10
#111 @ 51,168: 10x28
#112 @ 890,720: 17x23
#113 @ 538,52: 17x18
#114 @ 554,96: 24x19
#115 @ 628,811: 21x10
#116 @ 680,67: 15x13
#117 @ 327,267: 28x22
#118 @ 659,387: 14x20
#119 @ 796,10: 29x11
#120 @ 265,685: 29x13
#121 @ 724,457: 24x22
#122 @ 758,822: 18x13
#123 @ 28,585: 13x26
#124 @ 668,791: 18x28
#125 @ 964,555: 26x26
#126 @ 17,585: 21x15
#127 @ 972,834: 22x16
#128 @ 875,287: 21x19
#129 @ 608,224: 27x27
#130 @ 934,119: 15x15
#131 @ 865,229: 21x11
#132 @ 40,776: 11x29
#133 @ 872,704: 12x18
#134 @ 877,457: 23x16
#135 @ 809,904: 24x17
#136 @ 36,822: 27x24
#137 @ 61,906: 20x19
#138 @ 814,389: 17x23
#139 @ 61,290: 21x22
#140 @ 853,697: 28x26
#141 @ 208,401: 26x29
#142 @ 536,263: 17x28
#143 @ 45,503: 26x18
#144 @ 558,771: 19x12
#145 @ 691,65: 23x28
#146 @ 714,905: 18x26
#147 @ 56,71: 11x21
#148 @ 85,30: 11x21
#149 @ 688,674: 10x11
#150 @ 669,647: 13x22
#151 @ 769,189: 16x13
#152 @ 803,664: 12x16
#153 @ 857,165: 18x10
#154 @ 347,164: 19x15
#155 @ 297,845: 13x15
#156 @ 961,716: 29x17
#157 @ 942,829: 28x20
#158 @ 934,69: 12x10
#159 @ 367,265: 14x17
#160 @ 451,180: 16x20
#161 @ 758,929: 12x29
#162 @ 811,883: 25x27
#163 @ 277,850: 24x19
#164 @ 78,480: 20x17
#165 @ 443,219: 18x29
#166 @ 272,525: 13x21
#167 @ 711,249: 28x17
#168 @ 137,284: 5x5
#169 @ 862,853: 11x11
#170 @ 826,304: 15x27
#171 @ 256,978: 27x12
#172 @ 478,897: 15x14
#173 @ 932,312: 22x15
#174 @ 194,906: 18x26
#175 @ 384,448: 12x29
#176 @ 396,397: 15x29
#177 @ 975,7: 16x16
#178 @ 300,21: 14x17
#179 @ 579,243: 14x20
#180 @ 870,908: 12x24
#181 @ 294,469: 29x22
#182 @ 74,242: 19x25
#183 @ 0,414: 11x28
#184 @ 684,132: 15x26
#185 @ 726,581: 23x10
#186 @ 584,713: 16x26
#187 @ 870,816: 28x26
#188 @ 558,115: 21x11
#189 @ 501,712: 27x26
#190 @ 394,515: 12x27
#191 @ 836,613: 18x23
#192 @ 386,339: 21x29
#193 @ 597,345: 12x29
#194 @ 10,510: 14x26
#195 @ 571,782: 14x18
#196 @ 347,705: 10x19
#197 @ 887,353: 21x16
#198 @ 805,126: 17x29
#199 @ 314,104: 25x28
#200 @ 117,578: 21x22
#201 @ 291,125: 12x28
#202 @ 584,115: 26x17
#203 @ 100,209: 21x20
#204 @ 660,572: 12x23
#205 @ 194,806: 23x25
#206 @ 802,857: 11x10
#207 @ 223,122: 19x20
#208 @ 966,602: 15x21
#209 @ 855,946: 21x26
#210 @ 729,897: 13x21
#211 @ 126,573: 13x28
#212 @ 286,692: 18x11
#213 @ 543,25: 25x11
#214 @ 489,948: 17x11
#215 @ 941,247: 25x13
#216 @ 805,774: 11x28
#217 @ 423,716: 16x24
#218 @ 38,211: 24x11
#219 @ 876,364: 25x26
#220 @ 945,926: 14x18
#221 @ 386,138: 14x23
#222 @ 494,222: 27x14
#223 @ 687,474: 17x25
#224 @ 35,406: 17x13
#225 @ 768,912: 13x27
#226 @ 682,116: 12x26
#227 @ 913,966: 22x10
#228 @ 11,759: 14x10
#229 @ 130,277: 17x17
#230 @ 44,437: 11x28
#231 @ 551,895: 16x16
#232 @ 364,934: 27x10
#233 @ 342,129: 13x19
#234 @ 770,34: 25x29
#235 @ 712,482: 23x26
#236 @ 138,575: 28x21
#237 @ 265,966: 18x5
#238 @ 204,74: 24x10
#239 @ 935,901: 28x20
#240 @ 305,80: 21x22
#241 @ 183,465: 13x5
#242 @ 706,612: 20x17
#243 @ 556,770: 16x20
#244 @ 67,222: 21x24
#245 @ 259,897: 20x22
#246 @ 483,545: 11x15
#247 @ 799,217: 21x16
#248 @ 56,764: 21x27
#249 @ 64,339: 20x21
#250 @ 443,341: 15x24
#251 @ 334,843: 14x25
#252 @ 701,341: 27x26
#253 @ 799,143: 25x23
#254 @ 218,635: 21x20
#255 @ 376,839: 10x20
#256 @ 553,122: 14x14
#257 @ 215,61: 10x14
#258 @ 740,738: 20x29
#259 @ 205,393: 27x28
#260 @ 803,909: 10x16
#261 @ 631,198: 17x10
#262 @ 222,635: 19x26
#263 @ 323,318: 28x29
#264 @ 950,304: 28x27
#265 @ 367,937: 7x3
#266 @ 105,418: 27x14
#267 @ 155,987: 21x10
#268 @ 57,481: 10x27
#269 @ 6,673: 29x10
#270 @ 346,181: 21x23
#271 @ 916,473: 11x18
#272 @ 386,588: 18x23
#273 @ 971,773: 16x20
#274 @ 837,654: 18x29
#275 @ 733,38: 16x24
#276 @ 635,350: 19x23
#277 @ 54,187: 23x25
#278 @ 483,359: 15x27
#279 @ 9,153: 20x11
#280 @ 396,479: 26x25
#281 @ 321,111: 23x12
#282 @ 46,70: 26x24
#283 @ 943,179: 15x18
#284 @ 189,127: 14x28
#285 @ 552,773: 29x15
#286 @ 152,837: 27x24
#287 @ 970,701: 13x22
#288 @ 512,384: 11x12
#289 @ 345,125: 20x15
#290 @ 250,297: 20x17
#291 @ 354,287: 12x11
#292 @ 523,285: 17x10
#293 @ 581,841: 27x26
#294 @ 518,567: 26x19
#295 @ 946,759: 11x26
#296 @ 561,766: 25x13
#297 @ 283,65: 20x18
#298 @ 426,9: 23x29
#299 @ 609,739: 10x21
#300 @ 144,420: 13x16
#301 @ 800,890: 20x25
#302 @ 51,665: 14x22
#303 @ 303,111: 19x16
#304 @ 56,197: 26x19
#305 @ 51,415: 25x26
#306 @ 776,444: 21x11
#307 @ 2,760: 13x24
#308 @ 335,488: 29x29
#309 @ 720,923: 23x18
#310 @ 854,951: 13x21
#311 @ 53,255: 23x26
#312 @ 796,122: 12x16
#313 @ 559,317: 14x26
#314 @ 913,585: 10x19
#315 @ 353,441: 10x27
#316 @ 44,207: 18x25
#317 @ 122,127: 17x11
#318 @ 403,71: 29x26
#319 @ 638,47: 22x26
#320 @ 580,278: 18x14
#321 @ 399,799: 22x22
#322 @ 294,827: 10x22
#323 @ 798,240: 13x13
#324 @ 408,375: 29x23
#325 @ 197,977: 18x7
#326 @ 237,889: 26x18
#327 @ 510,389: 26x24
#328 @ 768,505: 19x29
#329 @ 646,378: 26x13
#330 @ 8,610: 21x17
#331 @ 928,536: 29x10
#332 @ 160,752: 23x14
#333 @ 879,975: 25x14
#334 @ 887,590: 14x10
#335 @ 82,878: 16x12
#336 @ 183,679: 26x27
#337 @ 505,780: 16x14
#338 @ 613,494: 26x26
#339 @ 166,930: 24x23
#340 @ 195,864: 16x28
#341 @ 361,537: 25x10
#342 @ 934,493: 24x21
#343 @ 339,504: 28x24
#344 @ 664,738: 16x14
#345 @ 861,916: 13x22
#346 @ 639,879: 15x20
#347 @ 963,877: 21x23
#348 @ 842,29: 24x20
#349 @ 717,438: 19x21
#350 @ 490,897: 13x21
#351 @ 254,902: 18x12
#352 @ 684,535: 22x18
#353 @ 283,131: 11x14
#354 @ 887,586: 15x22
#355 @ 34,322: 14x14
#356 @ 901,191: 18x28
#357 @ 866,739: 26x13
#358 @ 247,921: 11x26
#359 @ 884,816: 10x29
#360 @ 867,24: 23x21
#361 @ 558,270: 5x5
#362 @ 943,620: 14x14
#363 @ 102,756: 20x29
#364 @ 26,669: 27x11
#365 @ 248,79: 12x16
#366 @ 957,368: 4x8
#367 @ 149,495: 26x21
#368 @ 967,622: 10x16
#369 @ 0,283: 13x26
#370 @ 30,835: 11x28
#371 @ 930,648: 24x23
#372 @ 754,802: 14x27
#373 @ 207,728: 23x21
#374 @ 601,50: 27x22
#375 @ 145,466: 19x21
#376 @ 217,701: 18x26
#377 @ 352,67: 18x14
#378 @ 331,742: 24x12
#379 @ 936,868: 29x28
#380 @ 287,11: 14x27
#381 @ 561,721: 14x20
#382 @ 565,777: 26x11
#383 @ 23,783: 26x17
#384 @ 343,706: 12x29
#385 @ 218,379: 11x14
#386 @ 650,663: 29x17
#387 @ 762,961: 16x23
#388 @ 941,128: 23x25
#389 @ 478,871: 29x25
#390 @ 77,504: 17x23
#391 @ 636,739: 24x28
#392 @ 83,586: 12x26
#393 @ 385,749: 15x28
#394 @ 763,139: 20x13
#395 @ 372,868: 17x10
#396 @ 251,352: 24x10
#397 @ 432,288: 21x16
#398 @ 194,78: 20x27
#399 @ 146,609: 19x21
#400 @ 632,181: 29x21
#401 @ 900,708: 22x15
#402 @ 692,477: 21x21
#403 @ 429,639: 25x23
#404 @ 462,311: 24x22
#405 @ 21,324: 22x11
#406 @ 166,609: 25x27
#407 @ 591,934: 20x17
#408 @ 168,971: 21x23
#409 @ 395,25: 16x11
#410 @ 934,193: 22x29
#411 @ 557,47: 18x10
#412 @ 639,414: 29x27
#413 @ 870,392: 10x16
#414 @ 576,163: 22x23
#415 @ 24,399: 29x13
#416 @ 603,201: 28x20
#417 @ 300,212: 18x17
#418 @ 445,812: 18x22
#419 @ 209,726: 21x14
#420 @ 344,743: 13x18
#421 @ 80,108: 27x26
#422 @ 289,881: 17x21
#423 @ 432,18: 11x18
#424 @ 560,821: 15x14
#425 @ 299,508: 21x28
#426 @ 881,231: 18x19
#427 @ 418,12: 21x19
#428 @ 928,750: 24x11
#429 @ 904,506: 14x10
#430 @ 837,776: 10x13
#431 @ 66,865: 22x29
#432 @ 605,74: 25x15
#433 @ 398,801: 21x21
#434 @ 2,248: 22x13
#435 @ 182,277: 16x27
#436 @ 917,14: 16x13
#437 @ 58,226: 13x10
#438 @ 867,133: 28x11
#439 @ 690,477: 27x21
#440 @ 185,205: 10x22
#441 @ 338,285: 20x10
#442 @ 171,834: 12x20
#443 @ 260,958: 29x19
#444 @ 590,453: 20x13
#445 @ 315,320: 17x11
#446 @ 43,548: 18x13
#447 @ 911,632: 13x14
#448 @ 704,249: 13x25
#449 @ 517,407: 13x26
#450 @ 571,953: 29x25
#451 @ 569,351: 12x29
#452 @ 226,484: 26x12
#453 @ 692,602: 23x26
#454 @ 565,926: 17x15
#455 @ 745,113: 11x28
#456 @ 501,405: 11x13
#457 @ 11,580: 26x17
#458 @ 388,814: 23x22
#459 @ 476,295: 23x16
#460 @ 328,544: 19x12
#461 @ 608,399: 11x13
#462 @ 213,571: 13x25
#463 @ 435,834: 18x13
#464 @ 797,947: 21x21
#465 @ 856,32: 27x11
#466 @ 176,683: 27x21
#467 @ 4,178: 12x15
#468 @ 282,138: 29x27
#469 @ 568,105: 13x20
#470 @ 850,722: 20x14
#471 @ 735,443: 18x29
#472 @ 764,499: 13x14
#473 @ 416,647: 24x15
#474 @ 195,71: 15x10
#475 @ 435,709: 24x17
#476 @ 114,565: 22x27
#477 @ 574,509: 22x21
#478 @ 243,357: 12x12
#479 @ 455,229: 16x20
#480 @ 617,795: 18x26
#481 @ 664,328: 26x24
#482 @ 83,352: 14x14
#483 @ 225,620: 24x23
#484 @ 685,880: 21x18
#485 @ 110,724: 22x25
#486 @ 339,68: 14x28
#487 @ 841,786: 22x28
#488 @ 212,261: 19x11
#489 @ 690,695: 25x21
#490 @ 182,458: 27x28
#491 @ 862,162: 19x18
#492 @ 930,67: 21x15
#493 @ 536,972: 16x20
#494 @ 352,293: 13x11
#495 @ 39,87: 29x19
#496 @ 841,827: 15x17
#497 @ 392,381: 26x27
#498 @ 274,52: 20x11
#499 @ 120,778: 19x11
#500 @ 77,871: 12x8
#501 @ 909,229: 15x16
#502 @ 104,467: 22x15
#503 @ 7,751: 10x25
#504 @ 349,957: 29x10
#505 @ 617,22: 16x15
#506 @ 252,748: 12x29
#507 @ 229,509: 24x27
#508 @ 255,104: 28x16
#509 @ 851,894: 10x24
#510 @ 704,865: 21x29
#511 @ 803,198: 14x13
#512 @ 608,523: 18x25
#513 @ 291,693: 18x23
#514 @ 960,247: 19x18
#515 @ 716,21: 18x26
#516 @ 862,701: 29x26
#517 @ 278,793: 29x20
#518 @ 255,217: 5x9
#519 @ 5,523: 10x21
#520 @ 119,606: 13x16
#521 @ 226,248: 25x21
#522 @ 586,24: 12x24
#523 @ 859,397: 17x17
#524 @ 315,960: 20x29
#525 @ 932,107: 21x18
#526 @ 430,882: 20x15
#527 @ 187,268: 28x15
#528 @ 497,645: 23x10
#529 @ 531,875: 25x23
#530 @ 359,934: 10x28
#531 @ 900,122: 13x26
#532 @ 739,815: 22x24
#533 @ 312,499: 23x22
#534 @ 581,319: 26x19
#535 @ 366,919: 21x21
#536 @ 433,107: 28x29
#537 @ 414,872: 26x12
#538 @ 29,136: 29x28
#539 @ 429,697: 28x27
#540 @ 424,385: 18x23
#541 @ 430,815: 23x23
#542 @ 956,256: 23x22
#543 @ 136,629: 13x18
#544 @ 150,523: 18x18
#545 @ 247,369: 10x16
#546 @ 542,353: 28x18
#547 @ 579,799: 11x21
#548 @ 700,39: 17x14
#549 @ 977,358: 14x17
#550 @ 192,279: 10x27
#551 @ 669,719: 21x12
#552 @ 841,535: 26x26
#553 @ 459,87: 11x20
#554 @ 804,954: 27x23
#555 @ 784,197: 23x14
#556 @ 22,74: 25x20
#557 @ 211,883: 14x22
#558 @ 31,330: 20x18
#559 @ 842,248: 17x26
#560 @ 956,201: 20x10
#561 @ 756,338: 18x15
#562 @ 877,37: 11x12
#563 @ 917,138: 10x14
#564 @ 868,74: 16x27
#565 @ 666,426: 16x20
#566 @ 174,157: 26x20
#567 @ 705,355: 15x25
#568 @ 167,552: 12x17
#569 @ 791,222: 15x22
#570 @ 842,396: 22x14
#571 @ 702,299: 20x25
#572 @ 266,545: 21x16
#573 @ 740,50: 20x18
#574 @ 193,742: 24x22
#575 @ 959,258: 16x27
#576 @ 765,236: 20x11
#577 @ 641,449: 20x27
#578 @ 926,885: 12x26
#579 @ 334,516: 16x22
#580 @ 254,538: 23x26
#581 @ 732,493: 20x23
#582 @ 192,614: 11x28
#583 @ 583,523: 29x25
#584 @ 444,742: 25x22
#585 @ 952,808: 17x29
#586 @ 171,145: 26x14
#587 @ 948,401: 21x27
#588 @ 820,227: 29x25
#589 @ 84,451: 29x21
#590 @ 717,931: 25x15
#591 @ 703,868: 26x21
#592 @ 782,371: 29x22
#593 @ 467,107: 13x24
#594 @ 158,462: 25x10
#595 @ 418,414: 23x18
#596 @ 890,332: 29x24
#597 @ 42,304: 28x19
#598 @ 127,369: 14x14
#599 @ 948,466: 11x19
#600 @ 698,534: 29x12
#601 @ 805,230: 23x18
#602 @ 951,319: 11x24
#603 @ 26,956: 13x27
#604 @ 393,479: 13x25
#605 @ 233,670: 13x15
#606 @ 505,208: 11x27
#607 @ 682,23: 23x18
#608 @ 43,611: 25x10
#609 @ 45,312: 18x25
#610 @ 703,569: 13x19
#611 @ 469,491: 27x18
#612 @ 686,933: 10x13
#613 @ 659,827: 19x21
#614 @ 915,123: 12x28
#615 @ 417,439: 17x28
#616 @ 203,587: 20x11
#617 @ 7,254: 28x13
#618 @ 176,364: 24x19
#619 @ 939,454: 15x20
#620 @ 914,300: 20x10
#621 @ 639,207: 16x25
#622 @ 5,584: 23x26
#623 @ 784,26: 28x13
#624 @ 949,309: 20x13
#625 @ 386,748: 28x19
#626 @ 913,493: 13x14
#627 @ 553,239: 14x25
#628 @ 320,766: 15x20
#629 @ 446,290: 29x27
#630 @ 37,759: 24x28
#631 @ 699,491: 12x27
#632 @ 494,8: 29x29
#633 @ 811,926: 28x20
#634 @ 550,545: 15x13
#635 @ 23,600: 20x17
#636 @ 669,927: 15x22
#637 @ 413,340: 17x15
#638 @ 942,449: 10x14
#639 @ 809,189: 24x27
#640 @ 214,647: 18x15
#641 @ 535,821: 10x24
#642 @ 784,877: 6x12
#643 @ 135,451: 26x21
#644 @ 803,250: 20x16
#645 @ 567,403: 17x21
#646 @ 948,707: 15x18
#647 @ 308,892: 13x18
#648 @ 862,178: 10x24
#649 @ 105,913: 24x15
#650 @ 618,435: 24x23
#651 @ 406,301: 18x22
#652 @ 170,144: 18x10
#653 @ 593,194: 13x17
#654 @ 587,688: 12x17
#655 @ 526,8: 27x24
#656 @ 396,777: 26x25
#657 @ 863,967: 29x14
#658 @ 895,454: 29x11
#659 @ 770,781: 26x19
#660 @ 706,939: 14x17
#661 @ 133,629: 25x21
#662 @ 717,288: 22x10
#663 @ 135,386: 13x18
#664 @ 366,287: 16x13
#665 @ 481,289: 29x21
#666 @ 188,49: 24x16
#667 @ 575,926: 26x28
#668 @ 779,875: 29x19
#669 @ 82,876: 21x17
#670 @ 465,257: 13x22
#671 @ 119,164: 25x20
#672 @ 168,312: 27x14
#673 @ 576,691: 21x12
#674 @ 879,482: 18x28
#675 @ 318,78: 18x18
#676 @ 566,784: 18x18
#677 @ 93,268: 14x25
#678 @ 716,698: 27x19
#679 @ 656,855: 26x23
#680 @ 688,911: 16x27
#681 @ 354,64: 23x13
#682 @ 927,560: 24x10
#683 @ 552,834: 28x12
#684 @ 606,397: 17x19
#685 @ 402,93: 24x24
#686 @ 713,714: 28x20
#687 @ 393,31: 19x29
#688 @ 287,752: 3x6
#689 @ 956,817: 13x28
#690 @ 291,331: 15x24
#691 @ 563,728: 9x6
#692 @ 782,211: 14x28
#693 @ 831,571: 22x10
#694 @ 880,645: 20x21
#695 @ 218,27: 19x18
#696 @ 975,484: 23x12
#697 @ 62,663: 21x29
#698 @ 688,910: 26x25
#699 @ 261,850: 25x20
#700 @ 309,447: 24x28
#701 @ 472,820: 12x15
#702 @ 361,61: 16x14
#703 @ 424,661: 26x20
#704 @ 738,245: 26x26
#705 @ 533,841: 29x24
#706 @ 580,195: 24x11
#707 @ 735,115: 18x12
#708 @ 255,866: 27x22
#709 @ 695,208: 29x21
#710 @ 962,458: 10x20
#711 @ 344,816: 10x14
#712 @ 707,154: 26x13
#713 @ 413,483: 16x28
#714 @ 802,371: 11x16
#715 @ 969,588: 26x20
#716 @ 50,58: 11x29
#717 @ 239,663: 13x15
#718 @ 919,489: 23x24
#719 @ 363,935: 27x27
#720 @ 276,313: 28x26
#721 @ 183,243: 15x27
#722 @ 883,653: 13x6
#723 @ 284,92: 23x19
#724 @ 770,878: 10x12
#725 @ 560,853: 10x12
#726 @ 857,891: 19x19
#727 @ 152,565: 21x18
#728 @ 227,695: 29x26
#729 @ 744,850: 17x23
#730 @ 649,909: 14x17
#731 @ 686,181: 15x26
#732 @ 531,526: 14x22
#733 @ 411,784: 19x27
#734 @ 719,571: 26x14
#735 @ 938,305: 12x28
#736 @ 888,843: 15x16
#737 @ 664,711: 18x13
#738 @ 872,813: 20x17
#739 @ 757,209: 10x12
#740 @ 872,356: 16x14
#741 @ 434,928: 20x18
#742 @ 275,750: 19x11
#743 @ 77,504: 16x15
#744 @ 894,177: 22x27
#745 @ 687,259: 25x16
#746 @ 976,709: 3x9
#747 @ 81,27: 14x29
#748 @ 311,169: 18x20
#749 @ 293,330: 13x27
#750 @ 329,929: 20x19
#751 @ 794,838: 18x25
#752 @ 211,654: 17x13
#753 @ 239,565: 10x11
#754 @ 608,435: 29x16
#755 @ 179,459: 27x17
#756 @ 496,214: 16x28
#757 @ 368,544: 12x10
#758 @ 712,391: 25x10
#759 @ 629,881: 28x29
#760 @ 370,280: 24x19
#761 @ 571,850: 19x28
#762 @ 874,166: 24x14
#763 @ 228,977: 12x21
#764 @ 641,69: 17x14
#765 @ 798,720: 22x12
#766 @ 570,828: 18x17
#767 @ 685,932: 18x23
#768 @ 110,915: 26x11
#769 @ 677,918: 24x10
#770 @ 420,797: 18x14
#771 @ 423,841: 15x19
#772 @ 186,956: 13x16
#773 @ 957,676: 14x26
#774 @ 800,544: 15x19
#775 @ 604,33: 18x25
#776 @ 357,538: 12x26
#777 @ 52,531: 14x10
#778 @ 530,282: 15x12
#779 @ 185,256: 17x18
#780 @ 498,409: 28x16
#781 @ 446,169: 17x23
#782 @ 324,608: 13x25
#783 @ 619,76: 7x10
#784 @ 470,816: 21x12
#785 @ 621,654: 10x23
#786 @ 565,106: 28x26
#787 @ 556,268: 11x12
#788 @ 24,93: 16x20
#789 @ 671,163: 24x14
#790 @ 299,631: 20x29
#791 @ 530,844: 13x18
#792 @ 717,283: 26x24
#793 @ 339,926: 17x12
#794 @ 250,213: 19x24
#795 @ 76,675: 27x13
#796 @ 277,642: 27x23
#797 @ 78,104: 23x22
#798 @ 561,882: 15x28
#799 @ 159,138: 13x24
#800 @ 974,764: 26x13
#801 @ 450,715: 15x26
#802 @ 822,16: 16x16
#803 @ 912,209: 22x23
#804 @ 395,717: 29x17
#805 @ 751,421: 10x14
#806 @ 706,153: 17x26
#807 @ 957,332: 12x11
#808 @ 767,30: 14x17
#809 @ 572,179: 12x12
#810 @ 559,607: 13x27
#811 @ 888,270: 18x26
#812 @ 393,451: 22x17
#813 @ 628,447: 13x21
#814 @ 719,295: 29x14
#815 @ 203,690: 15x18
#816 @ 663,812: 20x25
#817 @ 784,884: 25x18
#818 @ 14,766: 17x29
#819 @ 883,481: 19x20
#820 @ 948,199: 20x18
#821 @ 346,497: 26x23
#822 @ 299,935: 24x20
#823 @ 620,363: 21x12
#824 @ 179,944: 20x15
#825 @ 707,52: 14x23
#826 @ 779,523: 11x24
#827 @ 369,47: 18x29
#828 @ 799,206: 12x23
#829 @ 261,750: 23x13
#830 @ 36,453: 23x13
#831 @ 565,945: 29x24
#832 @ 53,972: 22x18
#833 @ 187,304: 21x17
#834 @ 834,642: 16x27
#835 @ 527,413: 19x25
#836 @ 682,17: 10x12
#837 @ 200,591: 10x28
#838 @ 319,84: 23x17
#839 @ 229,518: 13x11
#840 @ 175,677: 16x16
#841 @ 383,810: 27x23
#842 @ 958,815: 10x22
#843 @ 358,319: 15x17
#844 @ 854,824: 13x15
#845 @ 453,715: 18x28
#846 @ 906,559: 22x14
#847 @ 305,123: 29x17
#848 @ 829,76: 10x10
#849 @ 937,625: 21x10
#850 @ 206,479: 12x29
#851 @ 105,23: 11x21
#852 @ 314,941: 15x14
#853 @ 506,91: 23x17
#854 @ 169,209: 8x5
#855 @ 228,305: 21x29
#856 @ 197,890: 17x14
#857 @ 142,389: 11x27
#858 @ 944,94: 14x24
#859 @ 833,649: 25x16
#860 @ 352,317: 28x28
#861 @ 696,477: 15x25
#862 @ 400,289: 24x24
#863 @ 572,301: 18x29
#864 @ 122,341: 19x28
#865 @ 173,147: 14x8
#866 @ 749,386: 25x14
#867 @ 883,486: 8x10
#868 @ 527,703: 25x19
#869 @ 632,92: 20x11
#870 @ 80,939: 23x20
#871 @ 179,144: 11x19
#872 @ 57,149: 23x26
#873 @ 228,897: 27x16
#874 @ 728,866: 26x28
#875 @ 804,406: 23x20
#876 @ 898,962: 14x27
#877 @ 933,943: 18x16
#878 @ 209,31: 20x23
#879 @ 646,738: 28x10
#880 @ 788,686: 24x13
#881 @ 187,927: 11x15
#882 @ 364,545: 14x11
#883 @ 763,517: 25x11
#884 @ 22,803: 16x29
#885 @ 617,528: 10x29
#886 @ 853,841: 15x15
#887 @ 174,612: 5x20
#888 @ 16,200: 27x13
#889 @ 743,83: 15x22
#890 @ 907,714: 26x10
#891 @ 558,961: 12x14
#892 @ 554,203: 28x20
#893 @ 125,154: 28x16
#894 @ 576,266: 23x16
#895 @ 689,615: 20x29
#896 @ 953,538: 15x11
#897 @ 411,734: 22x22
#898 @ 733,755: 13x21
#899 @ 157,359: 17x16
#900 @ 45,611: 23x21
#901 @ 617,243: 15x10
#902 @ 796,134: 21x18
#903 @ 243,562: 17x24
#904 @ 337,52: 19x27
#905 @ 194,975: 25x19
#906 @ 731,270: 24x13
#907 @ 422,973: 21x21
#908 @ 401,526: 25x24
#909 @ 549,604: 19x17
#910 @ 908,341: 28x10
#911 @ 348,150: 23x18
#912 @ 592,500: 24x19
#913 @ 210,675: 10x4
#914 @ 734,252: 13x14
#915 @ 251,709: 21x27
#916 @ 522,880: 10x16
#917 @ 417,416: 19x26
#918 @ 320,920: 27x16
#919 @ 567,450: 11x11
#920 @ 441,847: 15x21
#921 @ 382,301: 18x26
#922 @ 686,856: 25x12
#923 @ 122,721: 29x10
#924 @ 520,376: 16x22
#925 @ 798,221: 12x26
#926 @ 470,498: 16x15
#927 @ 891,820: 10x21
#928 @ 383,821: 17x28
#929 @ 984,643: 10x17
#930 @ 980,362: 12x13
#931 @ 222,905: 29x18
#932 @ 230,984: 5x7
#933 @ 79,639: 16x14
#934 @ 435,155: 11x28
#935 @ 485,309: 17x26
#936 @ 77,277: 15x24
#937 @ 114,602: 29x29
#938 @ 945,464: 21x24
#939 @ 569,259: 22x17
#940 @ 268,682: 19x17
#941 @ 605,227: 29x24
#942 @ 504,707: 28x19
#943 @ 611,726: 15x26
#944 @ 917,313: 28x26
#945 @ 250,344: 25x24
#946 @ 955,679: 16x26
#947 @ 6,143: 18x29
#948 @ 274,952: 26x29
#949 @ 924,357: 28x24
#950 @ 40,776: 27x10
#951 @ 828,591: 21x28
#952 @ 656,587: 27x10
#953 @ 413,654: 16x13
#954 @ 446,325: 15x21
#955 @ 464,438: 26x14
#956 @ 389,264: 22x24
#957 @ 164,207: 18x11
#958 @ 84,181: 11x23
#959 @ 182,362: 10x28
#960 @ 945,185: 20x25
#961 @ 205,616: 15x15
#962 @ 532,244: 28x16
#963 @ 432,726: 12x18
#964 @ 667,620: 14x13
#965 @ 905,288: 27x17
#966 @ 558,549: 15x13
#967 @ 102,135: 29x14
#968 @ 389,609: 21x21
#969 @ 929,614: 28x21
#970 @ 921,226: 22x18
#971 @ 801,608: 18x13
#972 @ 271,770: 24x27
#973 @ 844,676: 14x13
#974 @ 122,718: 23x25
#975 @ 569,816: 23x21
#976 @ 182,146: 14x12
#977 @ 592,458: 25x18
#978 @ 513,649: 27x18
#979 @ 122,713: 23x11
#980 @ 28,820: 11x18
#981 @ 550,288: 16x22
#982 @ 263,528: 25x25
#983 @ 743,811: 13x15
#984 @ 60,953: 18x20
#985 @ 112,751: 18x28
#986 @ 540,11: 16x16
#987 @ 394,279: 25x27
#988 @ 332,566: 25x20
#989 @ 873,706: 11x27
#990 @ 533,31: 11x27
#991 @ 941,330: 17x13
#992 @ 20,822: 24x23
#993 @ 43,534: 14x27
#994 @ 792,446: 29x28
#995 @ 437,934: 12x7
#996 @ 729,236: 19x14
#997 @ 780,235: 21x25
#998 @ 262,765: 24x29
#999 @ 417,637: 29x20
#1000 @ 773,249: 11x16
#1001 @ 701,29: 13x26
#1002 @ 123,48: 27x26
#1003 @ 506,803: 27x19
#1004 @ 77,117: 15x17
#1005 @ 405,799: 14x22
#1006 @ 898,473: 21x19
#1007 @ 342,198: 21x20
#1008 @ 883,272: 25x16
#1009 @ 699,658: 27x16
#1010 @ 421,953: 17x26
#1011 @ 501,895: 14x18
#1012 @ 324,450: 18x27
#1013 @ 113,198: 27x19
#1014 @ 563,919: 11x18
#1015 @ 261,931: 17x26
#1016 @ 778,213: 13x23
#1017 @ 652,424: 22x21
#1018 @ 279,803: 13x17
#1019 @ 782,36: 24x22
#1020 @ 946,238: 24x15
#1021 @ 716,465: 25x10
#1022 @ 5,435: 25x21
#1023 @ 882,372: 4x4
#1024 @ 832,88: 16x24
#1025 @ 646,473: 16x11
#1026 @ 700,217: 13x16
#1027 @ 74,274: 17x14
#1028 @ 864,236: 19x10
#1029 @ 775,964: 24x19
#1030 @ 211,59: 10x27
#1031 @ 421,661: 15x29
#1032 @ 313,733: 18x10
#1033 @ 736,471: 12x20
#1034 @ 31,815: 27x12
#1035 @ 854,34: 13x24
#1036 @ 704,274: 19x13
#1037 @ 669,402: 19x28
#1038 @ 156,351: 10x17
#1039 @ 729,2: 17x11
#1040 @ 940,604: 21x11
#1041 @ 909,954: 23x16
#1042 @ 716,390: 25x27
#1043 @ 330,874: 20x21
#1044 @ 62,643: 17x27
#1045 @ 912,638: 19x20
#1046 @ 686,825: 18x29
#1047 @ 450,856: 10x13
#1048 @ 672,600: 14x27
#1049 @ 278,889: 23x20
#1050 @ 53,456: 12x20
#1051 @ 9,270: 18x26
#1052 @ 596,648: 26x17
#1053 @ 460,713: 11x14
#1054 @ 156,523: 20x16
#1055 @ 17,65: 23x26
#1056 @ 348,367: 19x12
#1057 @ 112,734: 19x10
#1058 @ 894,199: 27x12
#1059 @ 811,956: 10x16
#1060 @ 682,679: 13x29
#1061 @ 311,688: 13x12
#1062 @ 143,426: 17x16
#1063 @ 37,764: 17x23
#1064 @ 272,103: 16x25
#1065 @ 192,477: 17x10
#1066 @ 416,120: 24x27
#1067 @ 699,593: 16x26
#1068 @ 334,620: 10x28
#1069 @ 714,27: 26x26
#1070 @ 619,832: 18x10
#1071 @ 250,530: 22x12
#1072 @ 607,9: 15x24
#1073 @ 279,548: 18x29
#1074 @ 935,757: 25x26
#1075 @ 190,210: 25x28
#1076 @ 945,331: 11x10
#1077 @ 384,752: 23x22
#1078 @ 657,742: 11x16
#1079 @ 304,463: 23x13
#1080 @ 578,938: 10x18
#1081 @ 127,711: 27x20
#1082 @ 91,645: 10x27
#1083 @ 721,244: 16x10
#1084 @ 715,456: 19x29
#1085 @ 64,478: 15x11
#1086 @ 658,659: 10x14
#1087 @ 866,246: 18x12
#1088 @ 226,901: 20x29
#1089 @ 731,1: 14x11
#1090 @ 472,252: 27x16
#1091 @ 882,945: 18x29
#1092 @ 802,929: 5x9
#1093 @ 110,612: 28x27
#1094 @ 85,750: 26x14
#1095 @ 95,356: 28x21
#1096 @ 595,693: 15x26
#1097 @ 65,890: 14x18
#1098 @ 954,366: 11x18
#1099 @ 26,354: 27x21
#1100 @ 86,469: 27x21
#1101 @ 783,899: 10x13
#1102 @ 635,880: 23x13
#1103 @ 246,842: 20x26
#1104 @ 476,554: 28x19
#1105 @ 960,781: 29x29
#1106 @ 794,219: 26x26
#1107 @ 801,560: 26x13
#1108 @ 589,911: 20x24
#1109 @ 152,577: 25x22
#1110 @ 131,584: 11x18
#1111 @ 432,154: 20x22
#1112 @ 914,962: 25x25
#1113 @ 448,437: 18x24
#1114 @ 805,465: 12x25
#1115 @ 541,3: 16x15
#1116 @ 75,869: 23x17
#1117 @ 959,423: 20x15
#1118 @ 73,948: 15x26
#1119 @ 247,983: 29x15
#1120 @ 375,830: 12x23
#1121 @ 975,542: 16x25
#1122 @ 206,637: 11x18
#1123 @ 773,38: 27x10
#1124 @ 817,926: 27x29
#1125 @ 875,142: 26x12
#1126 @ 581,417: 14x16
#1127 @ 462,716: 13x16
#1128 @ 570,916: 25x15
#1129 @ 208,883: 16x29
#1130 @ 170,299: 25x17
#1131 @ 620,241: 20x12
#1132 @ 969,236: 15x29
#1133 @ 250,260: 26x22
#1134 @ 30,89: 29x22
#1135 @ 336,923: 16x27
#1136 @ 191,54: 7x3
#1137 @ 470,151: 29x16
#1138 @ 368,321: 12x22
#1139 @ 73,754: 10x11
#1140 @ 336,882: 15x19
#1141 @ 105,294: 14x27
#1142 @ 220,112: 20x28
#1143 @ 607,365: 17x23
#1144 @ 365,847: 13x25
#1145 @ 13,87: 22x15
#1146 @ 699,815: 13x14
#1147 @ 80,668: 16x22
#1148 @ 494,223: 23x14
#1149 @ 242,381: 15x12
#1150 @ 942,853: 17x13
#1151 @ 410,708: 13x19
#1152 @ 302,309: 19x15
#1153 @ 365,324: 27x23
#1154 @ 571,436: 14x20
#1155 @ 130,771: 20x12
#1156 @ 226,324: 20x13
#1157 @ 858,348: 26x29
#1158 @ 856,354: 27x13
#1159 @ 746,66: 22x21
#1160 @ 382,537: 21x10
#1161 @ 954,710: 28x21
#1162 @ 107,301: 6x11
#1163 @ 661,811: 12x24
#1164 @ 269,910: 25x25
#1165 @ 188,946: 26x15
#1166 @ 446,24: 12x25
#1167 @ 431,735: 28x29
#1168 @ 227,374: 10x16
#1169 @ 245,70: 15x19
#1170 @ 77,950: 6x20
#1171 @ 369,961: 25x26
#1172 @ 44,172: 25x21
#1173 @ 76,104: 27x22
#1174 @ 335,552: 10x22
#1175 @ 683,168: 4x4
#1176 @ 968,616: 19x26
#1177 @ 559,857: 20x23
#1178 @ 82,862: 26x20
#1179 @ 727,288: 24x23
#1180 @ 961,692: 17x23
#1181 @ 311,226: 10x25
#1182 @ 935,652: 15x22
#1183 @ 392,800: 23x23
#1184 @ 173,159: 15x17
#1185 @ 952,759: 20x29
#1186 @ 191,933: 11x21
#1187 @ 288,126: 21x25
#1188 @ 747,389: 16x11
#1189 @ 566,322: 16x22
#1190 @ 739,323: 18x27
#1191 @ 705,259: 25x27
#1192 @ 250,474: 11x16
#1193 @ 149,122: 24x24
#1194 @ 703,127: 27x29
#1195 @ 952,711: 20x26
#1196 @ 67,502: 10x20
#1197 @ 330,825: 26x17
#1198 @ 950,243: 24x25
#1199 @ 331,540: 21x14
#1200 @ 445,101: 27x13
#1201 @ 397,745: 21x14
#1202 @ 661,0: 26x12
#1203 @ 356,59: 17x19
#1204 @ 369,433: 6x8
#1205 @ 497,773: 18x20
#1206 @ 509,379: 24x24
#1207 @ 45,174: 13x12
#1208 @ 302,830: 24x20
#1209 @ 125,631: 18x13
#1210 @ 949,240: 26x27
#1211 @ 957,697: 23x25
#1212 @ 58,101: 27x17
#1213 @ 440,926: 11x12
#1214 @ 208,411: 13x11
#1215 @ 395,77: 19x21
#1216 @ 225,363: 14x29
#1217 @ 706,42: 10x28
#1218 @ 114,454: 24x23
#1219 @ 229,890: 18x23
#1220 @ 522,30: 15x11
#1221 @ 157,156: 20x10
#1222 @ 531,705: 21x23
#1223 @ 207,673: 26x12
#1224 @ 26,822: 13x12
#1225 @ 72,651: 17x26
#1226 @ 443,589: 12x15
#1227 @ 267,297: 16x22
#1228 @ 609,441: 28x26
#1229 @ 775,154: 12x21
#1230 @ 442,907: 12x27
#1231 @ 333,490: 10x11
#1232 @ 214,390: 23x22
#1233 @ 385,357: 24x19
#1234 @ 185,281: 26x25
#1235 @ 196,63: 26x15
#1236 @ 872,944: 25x10
#1237 @ 495,555: 28x24
#1238 @ 668,313: 16x29
#1239 @ 802,215: 14x15
#1240 @ 168,488: 16x27
#1241 @ 812,675: 21x29
#1242 @ 804,238: 19x23
#1243 @ 321,834: 21x11
#1244 @ 428,706: 11x21
#1245 @ 590,213: 13x16
#1246 @ 959,666: 23x25
#1247 @ 194,71: 28x13
#1248 @ 604,823: 24x17
#1249 @ 693,583: 14x21
#1250 @ 462,175: 18x13
#1251 @ 424,143: 19x12
#1252 @ 961,669: 6x18
#1253 @ 861,489: 25x19
#1254 @ 419,652: 16x21
#1255 @ 329,857: 28x24
#1256 @ 138,598: 28x21
#1257 @ 12,758: 11x12
#1258 @ 868,118: 10x26
#1259 @ 915,599: 29x26
#1260 @ 969,774: 21x11
#1261 @ 703,713: 17x28
#1262 @ 757,201: 10x13
#1263 @ 580,261: 12x15
#1264 @ 538,953: 14x24
#1265 @ 306,722: 10x23
#1266 @ 310,504: 18x15
#1267 @ 331,457: 23x22
#1268 @ 65,390: 26x29
#1269 @ 720,914: 25x21
#1270 @ 230,663: 25x13
#1271 @ 266,80: 28x18
#1272 @ 632,457: 28x23
#1273 @ 135,461: 17x22
#1274 @ 53,408: 26x21
#1275 @ 286,900: 16x17
#1276 @ 483,557: 15x12
#1277 @ 125,359: 12x25
#1278 @ 718,862: 16x14
#1279 @ 529,819: 28x26
#1280 @ 278,885: 21x18
#1281 @ 839,707: 26x15
#1282 @ 860,9: 22x28
#1283 @ 226,654: 23x14
#1284 @ 177,167: 19x25
#1285 @ 218,382: 18x25
#1286 @ 133,32: 21x29
#1287 @ 798,240: 29x28
#1288 @ 412,401: 14x19
#1289 @ 91,946: 28x15
#1290 @ 758,880: 21x29
#1291 @ 843,570: 21x16
#1292 @ 375,334: 18x23
#1293 @ 12,161: 19x24
#1294 @ 862,89: 11x27
#1295 @ 377,599: 25x29
#1296 @ 208,57: 17x24
#1297 @ 386,333: 29x23
#1298 @ 33,437: 14x14
#1299 @ 439,126: 15x11
#1300 @ 35,98: 18x14
#1301 @ 44,913: 27x20
#1302 @ 917,593: 10x17
#1303 @ 737,300: 20x16
#1304 @ 63,183: 23x11
#1305 @ 352,223: 17x27
#1306 @ 794,446: 24x16
#1307 @ 462,640: 28x10
#1308 @ 366,61: 14x19
#1309 @ 73,125: 13x11
#1310 @ 453,154: 26x10
#1311 @ 707,151: 17x17
#1312 @ 591,34: 26x20
#1313 @ 968,830: 18x19
#1314 @ 818,66: 15x12
#1315 @ 127,366: 22x18
#1316 @ 644,215: 5x13
#1317 @ 387,536: 15x16
#1318 @ 938,363: 11x26
#1319 @ 800,916: 10x27
#1320 @ 657,856: 16x13
#1321 @ 316,889: 25x22
#1322 @ 940,854: 24x22
#1323 @ 306,697: 10x11
#1324 @ 422,656: 23x14
#1325 @ 866,812: 15x20
#1326 @ 569,55: 25x12
#1327 @ 947,333: 11x28
#1328 @ 814,924: 12x26
#1329 @ 652,9: 21x21
#1330 @ 365,279: 12x15
#1331 @ 232,626: 16x12
#1332 @ 482,364: 13x15
#1333 @ 69,240: 11x28
#1334 @ 751,293: 15x27
#1335 @ 879,782: 23x13
#1336 @ 82,359: 27x12
#1337 @ 330,618: 16x13
#1338 @ 788,774: 13x25
#1339 @ 39,205: 20x18
#1340 @ 366,961: 17x25
#1341 @ 729,414: 23x17
#1342 @ 60,404: 24x15
#1343 @ 88,565: 22x26
#1344 @ 799,730: 18x19
#1345 @ 486,629: 16x25
#1346 @ 180,635: 21x17
#1347 @ 753,119: 21x29
#1348 @ 83,588: 21x18
#1349 @ 724,871: 21x26";

    let s: Vec<String> = input.split('\n').map(String::from).collect();
    let my_split = |s: String| {
        s.split(|c| c == '#' || c == ' ' || c == '@' || c == ':' || c == ',' || c == 'x')
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.parse().ok())
            .collect()
    };
    let mut not_conflicting = HashSet::new();
    let mut m = HashMap::new();
    let s_to_map =
        |s: &Vec<i32>, m: &mut HashMap<(i32, i32), (i32, i32)>, nc: &mut HashSet<i32>| {
            nc.insert(s[0]);
            fill(m, (s[0], s[1], s[2], s[3], s[4]), nc);
        };
    for x in s {
        s_to_map(&(my_split(x.to_string())), &mut m, &mut not_conflicting);
    }
    println!("Part1: {:?}", &m.iter().filter(|(_, (_, v))| *v > 1).count());
    println!("Part2: {:?}", not_conflicting);
}
