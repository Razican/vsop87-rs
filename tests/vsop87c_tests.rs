use vsop87::*;

#[test]
fn it_mercury() {
    let coordinates = vsop87c::mercury(2451545.0);

    assert!(coordinates.x > -0.1300934113 && coordinates.x < -0.1300934111);
    assert!(coordinates.y > -0.4472876718 && coordinates.y < -0.4472876716);
    assert!(coordinates.z > -0.02459868 && coordinates.z < -0.02459808);

    let coordinates = vsop87c::mercury(2415020.0);

    assert!(coordinates.x > -0.3932698770 && coordinates.x < -0.3932698768);
    assert!(coordinates.y > -0.1406751044 && coordinates.y < -0.1406751042);
    assert!(coordinates.z > 0.02366133 && coordinates.z < 0.02366193);

    let coordinates = vsop87c::mercury(2378495.0);

    assert!(coordinates.x > -0.1548281759 && coordinates.x < -0.1548281757);
    assert!(coordinates.y > 0.2814058363 && coordinates.y < 0.2814058365);
    assert!(coordinates.z > 0.03769187 && coordinates.z < 0.03769247);

    let coordinates = vsop87c::mercury(2341970.0);

    assert!(coordinates.x > 0.3312331228 && coordinates.x < 0.3312331230);
    assert!(coordinates.y > 0.0640513913 && coordinates.y < 0.0640513915);
    assert!(coordinates.z > -0.02305957 && coordinates.z < -0.02305897);

    let coordinates = vsop87c::mercury(2305445.0);

    assert!(coordinates.x > 0.1950803106 && coordinates.x < 0.1950803108);
    assert!(coordinates.y > -0.3828531310 && coordinates.y < -0.3828531308);
    assert!(coordinates.z > -0.05054446 && coordinates.z < -0.05054386);

    let coordinates = vsop87c::mercury(2268920.0);

    assert!(coordinates.x > -0.2020188183 && coordinates.x < -0.2020188181);
    assert!(coordinates.y > -0.4195626842 && coordinates.y < -0.4195626840);
    assert!(coordinates.z > -0.02129831 && coordinates.z < -0.02129771);

    let coordinates = vsop87c::mercury(2232395.0);

    assert!(coordinates.x > -0.4084179068 && coordinates.x < -0.4084179066);
    assert!(coordinates.y > -0.0700940264 && coordinates.y < -0.0700940262);
    assert!(coordinates.z > 0.02654532 && coordinates.z < 0.02654592);

    let coordinates = vsop87c::mercury(2195870.0);

    assert!(coordinates.x > -0.0951835366 && coordinates.x < -0.0951835364);
    assert!(coordinates.y > 0.3043652762 && coordinates.y < 0.3043652764);
    assert!(coordinates.z > 0.03608555 && coordinates.z < 0.03608615);

    let coordinates = vsop87c::mercury(2159345.0);

    assert!(coordinates.x > 0.3404269125 && coordinates.x < 0.3404269127);
    assert!(coordinates.y > -0.0004094322 && coordinates.y < -0.0004094320);
    assert!(coordinates.z > -0.02622050 && coordinates.z < -0.02621990);

    let coordinates = vsop87c::mercury(2122820.0);

    assert!(coordinates.x > 0.1279588349 && coordinates.x < 0.1279588351);
    assert!(coordinates.y > -0.4129965857 && coordinates.y < -0.4129965855);
    assert!(coordinates.z > -0.04964807 && coordinates.z < -0.04964747);
}

#[test]
fn it_venus() {
    let coordinates = vsop87c::venus(2451545.0);

    assert!(coordinates.x > -0.7183022791 && coordinates.x < -0.7183022789);
    assert!(coordinates.y > -0.0326545996 && coordinates.y < -0.0326545994);
    assert!(coordinates.z > 0.04101400 && coordinates.z < 0.04101460);

    let coordinates = vsop87c::venus(2415020.0);

    assert!(coordinates.x > 0.6919778853 && coordinates.x < 0.6919778855);
    assert!(coordinates.y > -0.2203045664 && coordinates.y < -0.2203045662);
    assert!(coordinates.z > -0.04298775 && coordinates.z < -0.04298715);

    let coordinates = vsop87c::venus(2378495.0);

    assert!(coordinates.x > -0.5783536699 && coordinates.x < -0.5783536697);
    assert!(coordinates.y > 0.4245514324 && coordinates.y < 0.4245514326);
    assert!(coordinates.z > 0.03966557 && coordinates.z < 0.03966617);

    let coordinates = vsop87c::venus(2341970.0);

    assert!(coordinates.x > 0.4103332104 && coordinates.x < 0.4103332106);
    assert!(coordinates.y > -0.6008366750 && coordinates.y < -0.6008366748);
    assert!(coordinates.z > -0.03319967 && coordinates.z < -0.03319907);

    let coordinates = vsop87c::venus(2305445.0);

    assert!(coordinates.x > -0.1835053719 && coordinates.x < -0.1835053717);
    assert!(coordinates.y > 0.6944530426 && coordinates.y < 0.6944530428);
    assert!(coordinates.z > 0.02237403 && coordinates.z < 0.02237463);

    let coordinates = vsop87c::venus(2268920.0);

    assert!(coordinates.x > -0.0456738891 && coordinates.x < -0.0456738889);
    assert!(coordinates.y > -0.7258238141 && coordinates.y < -0.7258238139);
    assert!(coordinates.z > -0.01057813 && coordinates.z < -0.01057753);

    let coordinates = vsop87c::venus(2232395.0);

    assert!(coordinates.x > 0.2925184048 && coordinates.x < 0.2925184050);
    assert!(coordinates.y > 0.6584829435 && coordinates.y < 0.6584829437);
    assert!(coordinates.z > -0.00389847 && coordinates.z < -0.00389787);

    let coordinates = vsop87c::venus(2195870.0);

    assert!(coordinates.x > -0.4817622081 && coordinates.x < -0.4817622079);
    assert!(coordinates.y > -0.5412019870 && coordinates.y < -0.5412019868);
    assert!(coordinates.z > 0.01611251 && coordinates.z < 0.01611311);

    let coordinates = vsop87c::venus(2159345.0);

    assert!(coordinates.x > 0.6411201420 && coordinates.x < 0.6411201422);
    assert!(coordinates.y > 0.3341571870 && coordinates.y < 0.3341571872);
    assert!(coordinates.z > -0.02860935 && coordinates.z < -0.02860875);

    let coordinates = vsop87c::venus(2122820.0);

    assert!(coordinates.x > -0.7099110736 && coordinates.x < -0.7099110734);
    assert!(coordinates.y > -0.1240161269 && coordinates.y < -0.1240161267);
    assert!(coordinates.z > 0.03642526 && coordinates.z < 0.03642586);
}

#[test]
fn it_earth() {
    let coordinates = vsop87c::earth(2451545.0);

    assert!(coordinates.x > -0.1771354616 && coordinates.x < -0.1771354614);
    assert!(coordinates.y > 0.9672416228 && coordinates.y < 0.9672416230);
    assert!(coordinates.z > -0.00000420 && coordinates.z < -0.00000360);

    let coordinates = vsop87c::earth(2415020.0);

    assert!(coordinates.x > -0.1647294828 && coordinates.x < -0.1647294826);
    assert!(coordinates.y > 0.9693720023 && coordinates.y < 0.9693720025);
    assert!(coordinates.z > -0.00000086 && coordinates.z < -0.00000026);

    let coordinates = vsop87c::earth(2378495.0);

    assert!(coordinates.x > -0.1522449492 && coordinates.x < -0.1522449490);
    assert!(coordinates.y > 0.9713689618 && coordinates.y < 0.9713689620);
    assert!(coordinates.z > -0.00000010 && coordinates.z < 0.00000050);

    let coordinates = vsop87c::earth(2341970.0);

    assert!(coordinates.x > -0.1397668625 && coordinates.x < -0.1397668623);
    assert!(coordinates.y > 0.9731643442 && coordinates.y < 0.9731643444);
    assert!(coordinates.z > 0.00000216 && coordinates.z < 0.00000276);

    let coordinates = vsop87c::earth(2305445.0);

    assert!(coordinates.x > -0.1272616592 && coordinates.x < -0.1272616590);
    assert!(coordinates.y > 0.9748538869 && coordinates.y < 0.9748538871);
    assert!(coordinates.z > -0.00000191 && coordinates.z < -0.00000131);

    let coordinates = vsop87c::earth(2268920.0);

    assert!(coordinates.x > -0.1147252849 && coordinates.x < -0.1147252847);
    assert!(coordinates.y > 0.9763645264 && coordinates.y < 0.9763645266);
    assert!(coordinates.z > -0.00000230 && coordinates.z < -0.00000170);

    let coordinates = vsop87c::earth(2232395.0);

    assert!(coordinates.x > -0.1022656437 && coordinates.x < -0.1022656435);
    assert!(coordinates.y > 0.9777418150 && coordinates.y < 0.9777418152);
    assert!(coordinates.z > 0.00000342 && coordinates.z < 0.00000402);

    let coordinates = vsop87c::earth(2195870.0);

    assert!(coordinates.x > -0.0897381299 && coordinates.x < -0.0897381297);
    assert!(coordinates.y > 0.9789899622 && coordinates.y < 0.9789899624);
    assert!(coordinates.z > 0.00000119 && coordinates.z < 0.00000179);

    let coordinates = vsop87c::earth(2159345.0);

    assert!(coordinates.x > -0.0773033684 && coordinates.x < -0.0773033682);
    assert!(coordinates.y > 0.9799998883 && coordinates.y < 0.9799998885);
    assert!(coordinates.z > -0.00000158 && coordinates.z < -0.00000098);

    let coordinates = vsop87c::earth(2122820.0);

    assert!(coordinates.x > -0.0647576072 && coordinates.x < -0.0647576070);
    assert!(coordinates.y > 0.9808979020 && coordinates.y < 0.9808979022);
    assert!(coordinates.z > -0.00000338 && coordinates.z < -0.00000278);
}

#[test]
fn it_mars() {
    let coordinates = vsop87c::mars(2451545.0);

    assert!(coordinates.x > 1.3907159217 && coordinates.x < 1.3907159219);
    assert!(coordinates.y > -0.0134157061 && coordinates.y < -0.0134157059);
    assert!(coordinates.z > -0.03446810 && coordinates.z < -0.03446750);

    let coordinates = vsop87c::mars(2415020.0);

    assert!(coordinates.x > 0.3952725705 && coordinates.x < 0.3952725707);
    assert!(coordinates.y > -1.3652842041 && coordinates.y < -1.3652842039);
    assert!(coordinates.z > -0.03866575 && coordinates.z < -0.03866515);

    let coordinates = vsop87c::mars(2378495.0);

    assert!(coordinates.x > -1.1640169844 && coordinates.x < -1.1640169842);
    assert!(coordinates.y > -1.0408461241 && coordinates.y < -1.0408461239);
    assert!(coordinates.z > 0.00546030 && coordinates.z < 0.00546090);

    let coordinates = vsop87c::mars(2341970.0);

    assert!(coordinates.x > -1.6160583004 && coordinates.x < -1.6160583002);
    assert!(coordinates.y > 0.3697531113 && coordinates.y < 0.3697531115);
    assert!(coordinates.z > 0.04647523 && coordinates.z < 0.04647583);

    let coordinates = vsop87c::mars(2305445.0);

    assert!(coordinates.x > -0.6896577497 && coordinates.x < -0.6896577495);
    assert!(coordinates.y > 1.4840391394 && coordinates.y < 1.4840391396);
    assert!(coordinates.z > 0.04922323 && coordinates.z < 0.04922383);

    let coordinates = vsop87c::mars(2268920.0);

    assert!(coordinates.x > 0.8106181240 && coordinates.x < 0.8106181242);
    assert!(coordinates.y > 1.2767287233 && coordinates.y < 1.2767287235);
    assert!(coordinates.z > 0.01008347 && coordinates.z < 0.01008407);

    let coordinates = vsop87c::mars(2232395.0);

    assert!(coordinates.x > 1.3682927952 && coordinates.x < 1.3682927954);
    assert!(coordinates.y > -0.2563584218 && coordinates.y < -0.2563584216);
    assert!(coordinates.z > -0.03714956 && coordinates.z < -0.03714896);

    let coordinates = vsop87c::mars(2195870.0);

    assert!(coordinates.x > 0.1516988676 && coordinates.x < 0.1516988678);
    assert!(coordinates.y > -1.4122847591 && coordinates.y < -1.4122847589);
    assert!(coordinates.z > -0.03623311 && coordinates.z < -0.03623251);

    let coordinates = vsop87c::mars(2159345.0);

    assert!(coordinates.x > -1.3274727562 && coordinates.x < -1.3274727560);
    assert!(coordinates.y > -0.8181547192 && coordinates.y < -0.8181547190);
    assert!(coordinates.z > 0.01027368 && coordinates.z < 0.01027428);

    let coordinates = vsop87c::mars(2122820.0);

    assert!(coordinates.x > -1.5224566564 && coordinates.x < -1.5224566562);
    assert!(coordinates.y > 0.6524641788 && coordinates.y < 0.6524641790);
    assert!(coordinates.z > 0.04896365 && coordinates.z < 0.04896425);
}

#[test]
fn it_jupiter() {
    let coordinates = vsop87c::jupiter(2451545.0);

    assert!(coordinates.x > 4.0011739728 && coordinates.x < 4.0011739730);
    assert!(coordinates.y > 2.9385810259 && coordinates.y < 2.9385810261);
    assert!(coordinates.z > -0.10178405 && coordinates.z < -0.10178345);

    let coordinates = vsop87c::jupiter(2415020.0);

    assert!(coordinates.x > -3.1268885680 && coordinates.x < -3.1268885678);
    assert!(coordinates.y > -4.3833244170 && coordinates.y < -4.3833244168);
    assert!(coordinates.z > 0.08693536 && coordinates.z < 0.08693596);

    let coordinates = vsop87c::jupiter(2378495.0);

    assert!(coordinates.x > 0.2320147905 && coordinates.x < 0.2320147907);
    assert!(coordinates.y > 5.1265494301 && coordinates.y < 5.1265494303);
    assert!(coordinates.z > -0.02237864 && coordinates.z < -0.02237804);

    let coordinates = vsop87c::jupiter(2341970.0);

    assert!(coordinates.x > 0.9111045636 && coordinates.x < 0.9111045638);
    assert!(coordinates.y > -5.1081933868 && coordinates.y < -5.1081933866);
    assert!(coordinates.z > -0.00575888 && coordinates.z < -0.00575828);

    let coordinates = vsop87c::jupiter(2305445.0);

    assert!(coordinates.x > -3.6969935265 && coordinates.x < -3.6969935263);
    assert!(coordinates.y > 3.8580245749 && coordinates.y < 3.8580245751);
    assert!(coordinates.z > 0.07509245 && coordinates.z < 0.07509305);

    let coordinates = vsop87c::jupiter(2268920.0);

    assert!(coordinates.x > 4.3137648978 && coordinates.x < 4.3137648980);
    assert!(coordinates.y > -2.5299346445 && coordinates.y < -2.5299346443);
    assert!(coordinates.z > -0.09420178 && coordinates.z < -0.09420118);

    let coordinates = vsop87c::jupiter(2232395.0);

    assert!(coordinates.x > -5.4401549181 && coordinates.x < -5.4401549179);
    assert!(coordinates.y > 0.2866406675 && coordinates.y < 0.2866406677);
    assert!(coordinates.z > 0.12595009 && coordinates.z < 0.12595069);

    let coordinates = vsop87c::jupiter(2195870.0);

    assert!(coordinates.x > 4.6200396687 && coordinates.x < 4.6200396689);
    assert!(coordinates.y > 1.8327269763 && coordinates.y < 1.8327269765);
    assert!(coordinates.z > -0.11058180 && coordinates.z < -0.11058120);

    let coordinates = vsop87c::jupiter(2159345.0);

    assert!(coordinates.x > -4.1078797677 && coordinates.x < -4.1078797675);
    assert!(coordinates.y > -3.4875973205 && coordinates.y < -3.4875973203);
    assert!(coordinates.z > 0.10000079 && coordinates.z < 0.10000139);

    let coordinates = vsop87c::jupiter(2122820.0);

    assert!(coordinates.x > 1.5191368969 && coordinates.x < 1.5191368971);
    assert!(coordinates.y > 4.8886161889 && coordinates.y < 4.8886161891);
    assert!(coordinates.z > -0.03856696 && coordinates.z < -0.03856636);
}

#[test]
fn it_saturn() {
    let coordinates = vsop87c::saturn(2451545.0);

    assert!(coordinates.x > 6.4064067819 && coordinates.x < 6.4064067821);
    assert!(coordinates.y > 6.5699928469 && coordinates.y < 6.5699928471);
    assert!(coordinates.z > -0.36907710 && coordinates.z < -0.36907650);

    let coordinates = vsop87c::saturn(2415020.0);

    assert!(coordinates.x > -0.6146416434 && coordinates.x < -0.6146416432);
    assert!(coordinates.y > -10.0461993858 && coordinates.y < -10.0461993856);
    assert!(coordinates.z > 0.19397737 && coordinates.z < 0.19397797);

    let coordinates = vsop87c::saturn(2378495.0);

    assert!(coordinates.x > -5.3256669098 && coordinates.x < -5.3256669096);
    assert!(coordinates.y > 7.3835415373 && coordinates.y < 7.3835415375);
    assert!(coordinates.z > 0.09482534 && coordinates.z < 0.09482594);

    let coordinates = vsop87c::saturn(2341970.0);

    assert!(coordinates.x > 8.6927631229 && coordinates.x < 8.6927631231);
    assert!(coordinates.y > -4.4352082614 && coordinates.y < -4.4352082612);
    assert!(coordinates.z > -0.28452481 && coordinates.z < -0.28452421);

    let coordinates = vsop87c::saturn(2305445.0);

    assert!(coordinates.x > -9.0518360160 && coordinates.x < -9.0518360158);
    assert!(coordinates.y > -3.6171280352 && coordinates.y < -3.6171280350);
    assert!(coordinates.z > 0.42628357 && coordinates.z < 0.42628417);

    let coordinates = vsop87c::saturn(2268920.0);

    assert!(coordinates.x > 5.9153005787 && coordinates.x < 5.9153005789);
    assert!(coordinates.y > 6.8629464079 && coordinates.y < 6.8629464081);
    assert!(coordinates.z > -0.34387152 && coordinates.z < -0.34387092);

    let coordinates = vsop87c::saturn(2232395.0);

    assert!(coordinates.x > -0.2128973850 && coordinates.x < -0.2128973848);
    assert!(coordinates.y > -10.1032378667 && coordinates.y < -10.1032378665);
    assert!(coordinates.z > 0.14833043 && coordinates.z < 0.14833103);

    let coordinates = vsop87c::saturn(2195870.0);

    assert!(coordinates.x > -6.0841827844 && coordinates.x < -6.0841827842);
    assert!(coordinates.y > 6.8799721145 && coordinates.y < 6.8799721147);
    assert!(coordinates.z > 0.16398772 && coordinates.z < 0.16398832);

    let coordinates = vsop87c::saturn(2159345.0);

    assert!(coordinates.x > 8.7651857162 && coordinates.x < 8.7651857164);
    assert!(coordinates.y > -3.8844835883 && coordinates.y < -3.8844835881);
    assert!(coordinates.z > -0.32020948 && coordinates.z < -0.32020888);

    let coordinates = vsop87c::saturn(2122820.0);

    assert!(coordinates.x > -9.0192283868 && coordinates.x < -9.0192283866);
    assert!(coordinates.y > -3.9782745626 && coordinates.y < -3.9782745624);
    assert!(coordinates.z > 0.42944448 && coordinates.z < 0.42944508);
}

#[test]
fn it_uranus() {
    let coordinates = vsop87c::uranus(2451545.0);

    assert!(coordinates.x > 14.4318933504 && coordinates.x < 14.4318933506);
    assert!(coordinates.y > -13.7343158730 && coordinates.y < -13.7343158728);
    assert!(coordinates.z > -0.23814250 && coordinates.z < -0.23814190);

    let coordinates = vsop87c::uranus(2415020.0);

    assert!(coordinates.x > -6.9142977664 && coordinates.x < -6.9142977662);
    assert!(coordinates.y > -17.6894115249 && coordinates.y < -17.6894115247);
    assert!(coordinates.z > 0.02197490 && coordinates.z < 0.02197550);

    let coordinates = vsop87c::uranus(2378495.0);

    assert!(coordinates.x > -18.2009931565 && coordinates.x < -18.2009931563);
    assert!(coordinates.y > 1.8769054706 && coordinates.y < 1.8769054708);
    assert!(coordinates.z > 0.24225996 && coordinates.z < 0.24226056);

    let coordinates = vsop87c::uranus(2341970.0);

    assert!(coordinates.x > -2.8725042472 && coordinates.x < -2.8725042470);
    assert!(coordinates.y > 18.5754947896 && coordinates.y < 18.5754947898);
    assert!(coordinates.z > 0.11247159 && coordinates.z < 0.11247219);

    let coordinates = vsop87c::uranus(2305445.0);

    assert!(coordinates.x > 17.1435210735 && coordinates.x < 17.1435210737);
    assert!(coordinates.y > 9.8688935301 && coordinates.y < 9.8688935303);
    assert!(coordinates.z > -0.17800390 && coordinates.z < -0.17800330);

    let coordinates = vsop87c::uranus(2268920.0);

    assert!(coordinates.x > 16.5140119841 && coordinates.x < 16.5140119843);
    assert!(coordinates.y > -11.3323922782 && coordinates.y < -11.3323922780);
    assert!(coordinates.z > -0.25889591 && coordinates.z < -0.25889531);

    let coordinates = vsop87c::uranus(2232395.0);

    assert!(coordinates.x > -3.5812895194 && coordinates.x < -3.5812895192);
    assert!(coordinates.y > -18.9336732632 && coordinates.y < -18.9336732630);
    assert!(coordinates.z > -0.03719665 && coordinates.z < -0.03719605);

    let coordinates = vsop87c::uranus(2195870.0);

    assert!(coordinates.x > -18.2738161644 && coordinates.x < -18.2738161642);
    assert!(coordinates.y > -2.0946851369 && coordinates.y < -2.0946851367);
    assert!(coordinates.z > 0.22242351 && coordinates.z < 0.22242411);

    let coordinates = vsop87c::uranus(2159345.0);

    assert!(coordinates.x > -6.5914700063 && coordinates.x < -6.5914700061);
    assert!(coordinates.y > 17.3751653225 && coordinates.y < 17.3751653227);
    assert!(coordinates.z > 0.16362346 && coordinates.z < 0.16362406);

    let coordinates = vsop87c::uranus(2122820.0);

    assert!(coordinates.x > 14.9518051059 && coordinates.x < 14.9518051061);
    assert!(coordinates.y > 12.6122126487 && coordinates.y < 12.6122126489);
    assert!(coordinates.z > -0.12834172 && coordinates.z < -0.12834112);
}

#[test]
fn it_neptune() {
    let coordinates = vsop87c::neptune(2451545.0);

    assert!(coordinates.x > 16.8121111221 && coordinates.x < 16.8121111223);
    assert!(coordinates.y > -24.9916630359 && coordinates.y < -24.9916630357);
    assert!(coordinates.z > 0.12721872 && coordinates.z < 0.12721932);

    let coordinates = vsop87c::neptune(2415020.0);

    assert!(coordinates.x > 2.2429528711 && coordinates.x < 2.2429528713);
    assert!(coordinates.y > 29.7794830301 && coordinates.y < 29.7794830303);
    assert!(coordinates.z > -0.65594558 && coordinates.z < -0.65594498);

    let coordinates = vsop87c::neptune(2378495.0);

    assert!(coordinates.x > -21.3855253885 && coordinates.x < -21.3855253883);
    assert!(coordinates.y > -21.4739440136 && coordinates.y < -21.4739440134);
    assert!(coordinates.z > 0.94189524 && coordinates.z < 0.94189584);

    let coordinates = vsop87c::neptune(2341970.0);

    assert!(coordinates.x > 29.7592752325 && coordinates.x < 29.7592752327);
    assert!(coordinates.y > 2.4313629967 && coordinates.y < 2.4313629969);
    assert!(coordinates.z > -0.77874318 && coordinates.z < -0.77874258);

    let coordinates = vsop87c::neptune(2305445.0);

    assert!(coordinates.x > -25.0753375459 && coordinates.x < -25.0753375457);
    assert!(coordinates.y > 16.7126213835 && coordinates.y < 16.7126213837);
    assert!(coordinates.z > 0.30850885 && coordinates.z < 0.30850945);

    let coordinates = vsop87c::neptune(2268920.0);

    assert!(coordinates.x > 7.6259089975 && coordinates.x < 7.6259089977);
    assert!(coordinates.y > -29.1970035320 && coordinates.y < -29.1970035318);
    assert!(coordinates.z > 0.35280201 && coordinates.z < 0.35280261);

    let coordinates = vsop87c::neptune(2232395.0);

    assert!(coordinates.x > 12.1172445470 && coordinates.x < 12.1172445472);
    assert!(coordinates.y > 27.2486873863 && coordinates.y < 27.2486873865);
    assert!(coordinates.z > -0.81596275 && coordinates.z < -0.81596215);

    let coordinates = vsop87c::neptune(2195870.0);

    assert!(coordinates.x > -27.2598513120 && coordinates.x < -27.2598513118);
    assert!(coordinates.y > -13.2185335841 && coordinates.y < -13.2185335839);
    assert!(coordinates.z > 0.96032790 && coordinates.z < 0.96032849);

    let coordinates = vsop87c::neptune(2159345.0);

    assert!(coordinates.x > 28.8728659384 && coordinates.x < 28.8728659386);
    assert!(coordinates.y > -7.7682281057 && coordinates.y < -7.7682281055);
    assert!(coordinates.z > -0.64344943 && coordinates.z < -0.64344883);

    let coordinates = vsop87c::neptune(2122820.0);

    assert!(coordinates.x > -17.9958638452 && coordinates.x < -17.9958638450);
    assert!(coordinates.y > 24.0846111038 && coordinates.y < 24.0846111040);
    assert!(coordinates.z > 0.08267368 && coordinates.z < 0.08267428);
}
