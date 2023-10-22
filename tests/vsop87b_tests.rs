use vsop87::*;

#[test]
fn it_mercury() {
    let coordinates = vsop87b::mercury(2451545.0);

    assert!(coordinates.longitude() > 4.4293481042 && coordinates.longitude() < 4.4293481044);
    assert!(coordinates.latitude() > -0.0527573412 && coordinates.latitude() < -0.0527573410);
    assert!(coordinates.distance() > 0.4664711 && coordinates.distance() < 0.4664719);

    let coordinates = vsop87b::mercury(2415020.0);

    assert!(coordinates.longitude() > 3.5095041511 && coordinates.longitude() < 3.5095041513);
    assert!(coordinates.latitude() > 0.0564907882 && coordinates.latitude() < 0.0564907884);
    assert!(coordinates.distance() > 0.4183422 && coordinates.distance() < 0.4183430);

    let coordinates = vsop87b::mercury(2378495.0);

    assert!(coordinates.longitude() > 2.1225631483 && coordinates.longitude() < 2.1225631485);
    assert!(coordinates.latitude() > 0.1171864613 && coordinates.latitude() < 0.1171864615);
    assert!(coordinates.distance() > 0.3233906 && coordinates.distance() < 0.3233914);

    let coordinates = vsop87b::mercury(2341970.0);

    assert!(coordinates.longitude() > 0.2641557553 && coordinates.longitude() < 0.2641557555);
    assert!(coordinates.latitude() > -0.0680150538 && coordinates.latitude() < -0.0680150536);
    assert!(coordinates.distance() > 0.3381559 && coordinates.distance() < 0.3381567);

    let coordinates = vsop87b::mercury(2305445.0);

    assert!(coordinates.longitude() > 5.2811474960 && coordinates.longitude() < 5.2811474962);
    assert!(coordinates.latitude() > -0.1178238227 && coordinates.latitude() < -0.1178238225);
    assert!(coordinates.distance() > 0.4326514 && coordinates.distance() < 0.4326522);

    let coordinates = vsop87b::mercury(2268920.0);

    assert!(coordinates.longitude() > 4.3854123463 && coordinates.longitude() < 4.3854123465);
    assert!(coordinates.latitude() > -0.0468100866 && coordinates.latitude() < -0.0468100864);
    assert!(coordinates.distance() > 0.4661520 && coordinates.distance() < 0.4661528);

    let coordinates = vsop87b::mercury(2232395.0);

    assert!(coordinates.longitude() > 3.4577380613 && coordinates.longitude() < 3.4577380615);
    assert!(coordinates.latitude() > 0.0634626329 && coordinates.latitude() < 0.0634626331);
    assert!(coordinates.distance() > 0.4152381 && coordinates.distance() < 0.4152389);

    let coordinates = vsop87b::mercury(2195870.0);

    assert!(coordinates.longitude() > 2.0443901594 && coordinates.longitude() < 2.0443901596);
    assert!(coordinates.latitude() > 0.1140574169 && coordinates.latitude() < 0.1140574171);
    assert!(coordinates.distance() > 0.3209362 && coordinates.distance() < 0.3209370);

    let coordinates = vsop87b::mercury(2159345.0);

    assert!(coordinates.longitude() > 0.1936433213 && coordinates.longitude() < 0.1936433215);
    assert!(coordinates.latitude() > -0.0764174093 && coordinates.latitude() < -0.0764174091);
    assert!(coordinates.distance() > 0.3414350 && coordinates.distance() < 0.3414358);

    let coordinates = vsop87b::mercury(2122820.0);

    assert!(coordinates.longitude() > 5.2319689070 && coordinates.longitude() < 5.2319689072);
    assert!(coordinates.latitude() > -0.1160635433 && coordinates.latitude() < -0.1160635431);
    assert!(coordinates.distance() > 0.4352059 && coordinates.distance() < 0.4352067);
}

#[test]
fn it_venus() {
    let coordinates = vsop87b::venus(2451545.0);

    assert!(coordinates.longitude() > 3.1870221909 && coordinates.longitude() < 3.1870221911);
    assert!(coordinates.latitude() > 0.0569782848 && coordinates.latitude() < 0.0569782850);
    assert!(coordinates.distance() > 0.7202125 && coordinates.distance() < 0.7202133);

    let coordinates = vsop87b::venus(2415020.0);

    assert!(coordinates.longitude() > 5.9993518123 && coordinates.longitude() < 5.9993518125);
    assert!(coordinates.latitude() > -0.0591709805 && coordinates.latitude() < -0.0591709803);
    assert!(coordinates.distance() > 0.7274715 && coordinates.distance() < 0.7274723);

    let coordinates = vsop87b::venus(2378495.0);

    assert!(coordinates.longitude() > 2.5571297502 && coordinates.longitude() < 2.5571297504);
    assert!(coordinates.latitude() > 0.0554510890 && coordinates.latitude() < 0.0554510892);
    assert!(coordinates.distance() > 0.7185469 && coordinates.distance() < 0.7185477);

    let coordinates = vsop87b::venus(2341970.0);

    assert!(coordinates.longitude() > 5.3846889523 && coordinates.longitude() < 5.3846889525);
    assert!(coordinates.latitude() > -0.0460995954 && coordinates.latitude() < -0.0460995952);
    assert!(coordinates.distance() > 0.7283404 && coordinates.distance() < 0.7283412);

    let coordinates = vsop87b::venus(2305445.0);

    assert!(coordinates.longitude() > 1.9265887456 && coordinates.longitude() < 1.9265887458);
    assert!(coordinates.latitude() > 0.0319707651 && coordinates.latitude() < 0.0319707653);
    assert!(coordinates.distance() > 0.7186371 && coordinates.distance() < 0.7186379);

    let coordinates = vsop87b::venus(2268920.0);

    assert!(coordinates.longitude() > 4.7713211614 && coordinates.longitude() < 4.7713211616);
    assert!(coordinates.latitude() > -0.0156777293 && coordinates.latitude() < -0.0156777291);
    assert!(coordinates.distance() > 0.7273360 && coordinates.distance() < 0.7273368);

    let coordinates = vsop87b::venus(2232395.0);

    assert!(coordinates.longitude() > 1.2988483957 && coordinates.longitude() < 1.2988483959);
    assert!(coordinates.latitude() > -0.0040667686 && coordinates.latitude() < -0.0040667684);
    assert!(coordinates.distance() > 0.7205425 && coordinates.distance() < 0.7205433);

    let coordinates = vsop87b::venus(2195870.0);

    assert!(coordinates.longitude() > 4.1554559279 && coordinates.longitude() < 4.1554559281);
    assert!(coordinates.latitude() > 0.0208254121 && coordinates.latitude() < 0.0208254123);
    assert!(coordinates.distance() > 0.7247437 && coordinates.distance() < 0.7247445);

    let coordinates = vsop87b::venus(2159345.0);

    assert!(coordinates.longitude() > 0.6752327773 && coordinates.longitude() < 0.6752327775);
    assert!(coordinates.latitude() > -0.0383268979 && coordinates.latitude() < -0.0383268977);
    assert!(coordinates.distance() > 0.7235426 && coordinates.distance() < 0.7235434);

    let coordinates = vsop87b::venus(2122820.0);

    assert!(coordinates.longitude() > 3.5336333774 && coordinates.longitude() < 3.5336333776);
    assert!(coordinates.latitude() > 0.0496161271 && coordinates.latitude() < 0.0496161273);
    assert!(coordinates.distance() > 0.7215816 && coordinates.distance() < 0.7215824);
}

#[test]
fn it_earth() {
    let coordinates = vsop87b::earth(2451545.0);

    assert!(coordinates.longitude() > 1.7519238636 && coordinates.longitude() < 1.7519238638);
    assert!(coordinates.latitude() > -0.0000039657 && coordinates.latitude() < -0.0000039655);
    assert!(coordinates.distance() > 0.9833273 && coordinates.distance() < 0.9833281);

    let coordinates = vsop87b::earth(2415020.0);

    assert!(coordinates.longitude() > 1.7634989197 && coordinates.longitude() < 1.7634989199);
    assert!(coordinates.latitude() > 0.0002186909 && coordinates.latitude() < 0.0002186911);
    assert!(coordinates.distance() > 0.9832686 && coordinates.distance() < 0.9832694);

    let coordinates = vsop87b::earth(2378495.0);

    assert!(coordinates.longitude() > 1.7750058557 && coordinates.longitude() < 1.7750058559);
    assert!(coordinates.latitude() > 0.0004381094 && coordinates.latitude() < 0.0004381096);
    assert!(coordinates.distance() > 0.9832270 && coordinates.distance() < 0.9832278);

    let coordinates = vsop87b::earth(2341970.0);

    assert!(coordinates.longitude() > 1.7865387213 && coordinates.longitude() < 1.7865387215);
    assert!(coordinates.latitude() > 0.0006583864 && coordinates.latitude() < 0.0006583866);
    assert!(coordinates.distance() > 0.9831494 && coordinates.distance() < 0.9831502);

    let coordinates = vsop87b::earth(2305445.0);

    assert!(coordinates.longitude() > 1.7980474964 && coordinates.longitude() < 1.7980474966);
    assert!(coordinates.latitude() > 0.0008715328 && coordinates.latitude() < 0.0008715330);
    assert!(coordinates.distance() > 0.9831250 && coordinates.distance() < 0.9831258);

    let coordinates = vsop87b::earth(2268920.0);

    assert!(coordinates.longitude() > 1.8095367658 && coordinates.longitude() < 1.8095367660);
    assert!(coordinates.latitude() > 0.0010876905 && coordinates.latitude() < 0.0010876907);
    assert!(coordinates.distance() > 0.9830813 && coordinates.distance() < 0.9830821);

    let coordinates = vsop87b::earth(2232395.0);

    assert!(coordinates.longitude() > 1.8211080284 && coordinates.longitude() < 1.8211080286);
    assert!(coordinates.latitude() > 0.0013092602 && coordinates.latitude() < 0.0013092604);
    assert!(coordinates.distance() > 0.9830750 && coordinates.distance() < 0.9830758);

    let coordinates = vsop87b::earth(2195870.0);

    assert!(coordinates.longitude() > 1.8326137390 && coordinates.longitude() < 1.8326137392);
    assert!(coordinates.latitude() > 0.0015219400 && coordinates.latitude() < 0.0015219402);
    assert!(coordinates.distance() > 0.9830938 && coordinates.distance() < 0.9830946);

    let coordinates = vsop87b::earth(2159345.0);

    assert!(coordinates.longitude() > 1.8442244562 && coordinates.longitude() < 1.8442244564);
    assert!(coordinates.latitude() > 0.0017331614 && coordinates.latitude() < 0.0017331616);
    assert!(coordinates.distance() > 0.9830436 && coordinates.distance() < 0.9830444);

    let coordinates = vsop87b::earth(2122820.0);

    assert!(coordinates.longitude() > 1.8557201151 && coordinates.longitude() < 1.8557201153);
    assert!(coordinates.latitude() > 0.0019445313 && coordinates.latitude() < 0.0019445315);
    assert!(coordinates.distance() > 0.9830328 && coordinates.distance() < 0.9830336);
}

#[test]
fn it_mars() {
    let coordinates = vsop87b::mars(2451545.0);

    assert!(coordinates.longitude() > 6.2735389871 && coordinates.longitude() < 6.2735389873);
    assert!(coordinates.latitude() > -0.0247779825 && coordinates.latitude() < -0.0247779823);
    assert!(coordinates.distance() > 1.3912073 && coordinates.distance() < 1.3912081);

    let coordinates = vsop87b::mars(2415020.0);

    assert!(coordinates.longitude() > 5.0185792655 && coordinates.longitude() < 5.0185792657);
    assert!(coordinates.latitude() > -0.0274073501 && coordinates.latitude() < -0.0274073499);
    assert!(coordinates.distance() > 1.4218774 && coordinates.distance() < 1.4218782);

    let coordinates = vsop87b::mars(2378495.0);

    assert!(coordinates.longitude() > 3.9199284824 && coordinates.longitude() < 3.9199284826);
    assert!(coordinates.latitude() > 0.0031513364 && coordinates.latitude() < 0.0031513366);
    assert!(coordinates.distance() > 1.5615136 && coordinates.distance() < 1.5615144);

    let coordinates = vsop87b::mars(2341970.0);

    assert!(coordinates.longitude() > 2.9897807829 && coordinates.longitude() < 2.9897807831);
    assert!(coordinates.latitude() > 0.0280781216 && coordinates.latitude() < 0.0280781218);
    assert!(coordinates.distance() > 1.6584693 && coordinates.distance() < 1.6584701);

    let coordinates = vsop87b::mars(2305445.0);

    assert!(coordinates.longitude() > 2.1032776582 && coordinates.longitude() < 2.1032776584);
    assert!(coordinates.latitude() > 0.0308218938 && coordinates.latitude() < 0.0308218940);
    assert!(coordinates.distance() > 1.6371993 && coordinates.distance() < 1.6372001);

    let coordinates = vsop87b::mars(2268920.0);

    assert!(coordinates.longitude() > 1.1268677423 && coordinates.longitude() < 1.1268677425);
    assert!(coordinates.latitude() > 0.0077311847 && coordinates.latitude() < 0.0077311849);
    assert!(coordinates.distance() > 1.5123619 && coordinates.distance() < 1.5123627);

    let coordinates = vsop87b::mars(2232395.0);

    assert!(coordinates.longitude() > 6.2441093265 && coordinates.longitude() < 6.2441093267);
    assert!(coordinates.latitude() > -0.0266449541 && coordinates.latitude() < -0.0266449539);
    assert!(coordinates.distance() > 1.3925960 && coordinates.distance() < 1.3925968);

    let coordinates = vsop87b::mars(2195870.0);

    assert!(coordinates.longitude() > 4.9898149166 && coordinates.longitude() < 4.9898149168);
    assert!(coordinates.latitude() > -0.0270155267 && coordinates.latitude() < -0.0270155265);
    assert!(coordinates.distance() > 1.4208703 && coordinates.distance() < 1.4208711);

    let coordinates = vsop87b::mars(2159345.0);

    assert!(coordinates.longitude() > 3.8886466317 && coordinates.longitude() < 3.8886466319);
    assert!(coordinates.latitude() > 0.0052701266 && coordinates.latitude() < 0.0052701268);
    assert!(coordinates.distance() > 1.5593798 && coordinates.distance() < 1.5593806);

    let coordinates = vsop87b::mars(2122820.0);

    assert!(coordinates.longitude() > 2.9557712522 && coordinates.longitude() < 2.9557712524);
    assert!(coordinates.latitude() > 0.0298285523 && coordinates.latitude() < 0.0298285525);
    assert!(coordinates.distance() > 1.6570998 && coordinates.distance() < 1.6571006);
}

#[test]
fn it_jupiter() {
    let coordinates = vsop87b::jupiter(2451545.0);

    assert!(coordinates.longitude() > 0.6334614216 && coordinates.longitude() < 0.6334614218);
    assert!(coordinates.latitude() > -0.0205001040 && coordinates.latitude() < -0.0205001038);
    assert!(coordinates.distance() > 4.9653809 && coordinates.distance() < 4.9653817);

    let coordinates = vsop87b::jupiter(2415020.0);

    assert!(coordinates.longitude() > 4.1171308453 && coordinates.longitude() < 4.1171308455);
    assert!(coordinates.latitude() > 0.0159456649 && coordinates.latitude() < 0.0159456651);
    assert!(coordinates.distance() > 5.3850272 && coordinates.distance() < 5.3850280);

    let coordinates = vsop87b::jupiter(2378495.0);

    assert!(coordinates.longitude() > 1.5743114743 && coordinates.longitude() < 1.5743114745);
    assert!(coordinates.latitude() > -0.0039059815 && coordinates.latitude() < -0.0039059813);
    assert!(coordinates.distance() > 5.1318453 && coordinates.distance() < 5.1318461);

    let coordinates = vsop87b::jupiter(2341970.0);

    assert!(coordinates.longitude() > 4.9619913551 && coordinates.longitude() < 4.9619913553);
    assert!(coordinates.latitude() > -0.0017586235 && coordinates.latitude() < -0.0017586233);
    assert!(coordinates.distance() > 5.1888130 && coordinates.distance() < 5.1888138);

    let coordinates = vsop87b::jupiter(2305445.0);

    assert!(coordinates.longitude() > 2.4323346133 && coordinates.longitude() < 2.4323346135);
    assert!(coordinates.latitude() > 0.0145957281 && coordinates.latitude() < 0.0145957283);
    assert!(coordinates.distance() > 5.3439451 && coordinates.distance() < 5.3439459);

    let coordinates = vsop87b::jupiter(2268920.0);

    assert!(coordinates.longitude() > 5.8745612667 && coordinates.longitude() < 5.8745612669);
    assert!(coordinates.latitude() > -0.0192161118 && coordinates.latitude() < -0.0192161116);
    assert!(coordinates.distance() > 5.0018003 && coordinates.distance() < 5.0018011);

    let coordinates = vsop87b::jupiter(2232395.0);

    assert!(coordinates.longitude() > 3.2350793730 && coordinates.longitude() < 3.2350793732);
    assert!(coordinates.latitude() > 0.0229002243 && coordinates.latitude() < 0.0229002245);
    assert!(coordinates.distance() > 5.4491566 && coordinates.distance() < 5.4491574);

    let coordinates = vsop87b::jupiter(2195870.0);

    assert!(coordinates.longitude() > 0.5480874611 && coordinates.longitude() < 0.5480874613);
    assert!(coordinates.latitude() > -0.0213293087 && coordinates.latitude() < -0.0213293085);
    assert!(coordinates.distance() > 4.9715067 && coordinates.distance() < 4.9715075);

    let coordinates = vsop87b::jupiter(2159345.0);

    assert!(coordinates.longitude() > 4.0402354040 && coordinates.longitude() < 4.0402354042);
    assert!(coordinates.latitude() > 0.0170598704 && coordinates.latitude() < 0.0170598706);
    assert!(coordinates.distance() > 5.3896203 && coordinates.distance() < 5.3896211);

    let coordinates = vsop87b::jupiter(2122820.0);

    assert!(coordinates.longitude() > 1.4885071579 && coordinates.longitude() < 1.4885071581);
    assert!(coordinates.latitude() > -0.0054711801 && coordinates.latitude() < -0.0054711799);
    assert!(coordinates.distance() > 5.1193583 && coordinates.distance() < 5.1193591);
}

#[test]
fn it_saturn() {
    let coordinates = vsop87b::saturn(2451545.0);

    assert!(coordinates.longitude() > 0.7980038866 && coordinates.longitude() < 0.7980038868);
    assert!(coordinates.latitude() > -0.0401984150 && coordinates.latitude() < -0.0401984148);
    assert!(coordinates.distance() > 9.1838479 && coordinates.distance() < 9.1838487);

    let coordinates = vsop87b::saturn(2415020.0);

    assert!(coordinates.longitude() > 4.6756597985 && coordinates.longitude() < 4.6756597987);
    assert!(coordinates.latitude() > 0.0190423975 && coordinates.latitude() < 0.0190423977);
    assert!(coordinates.distance() > 10.0668528 && coordinates.distance() < 10.0668536);

    let coordinates = vsop87b::saturn(2378495.0);

    assert!(coordinates.longitude() > 2.2444130057 && coordinates.longitude() < 2.2444130059);
    assert!(coordinates.latitude() > 0.0107481007 && coordinates.latitude() < 0.0107481009);
    assert!(coordinates.distance() > 9.1043064 && coordinates.distance() < 9.1043072);

    let coordinates = vsop87b::saturn(2341970.0);

    assert!(coordinates.longitude() > 5.8845121484 && coordinates.longitude() < 5.8845121486);
    assert!(coordinates.latitude() > -0.0293639469 && coordinates.latitude() < -0.0293639467);
    assert!(coordinates.distance() > 9.7629991 && coordinates.distance() < 9.7629999);

    let coordinates = vsop87b::saturn(2305445.0);

    assert!(coordinates.longitude() > 3.6192301827 && coordinates.longitude() < 3.6192301829);
    assert!(coordinates.latitude() > 0.0432255906 && coordinates.latitude() < 0.0432255908);
    assert!(coordinates.distance() > 9.7571031 && coordinates.distance() < 9.7571039);

    let coordinates = vsop87b::saturn(2268920.0);

    assert!(coordinates.longitude() > 0.9812189104 && coordinates.longitude() < 0.9812189106);
    assert!(coordinates.latitude() > -0.0369435534 && coordinates.latitude() < -0.0369435532);
    assert!(coordinates.distance() > 9.0669210 && coordinates.distance() < 9.0669218);

    let coordinates = vsop87b::saturn(2232395.0);

    assert!(coordinates.longitude() > 4.8374129244 && coordinates.longitude() < 4.8374129246);
    assert!(coordinates.latitude() > 0.0133288783 && coordinates.latitude() < 0.0133288785);
    assert!(coordinates.distance() > 10.1065689 && coordinates.distance() < 10.1065697);

    let coordinates = vsop87b::saturn(2195870.0);

    assert!(coordinates.longitude() > 2.4653200324 && coordinates.longitude() < 2.4653200326);
    assert!(coordinates.latitude() > 0.0187797597 && coordinates.latitude() < 0.0187797599);
    assert!(coordinates.distance() > 9.1857595 && coordinates.distance() < 9.1857603);

    let coordinates = vsop87b::saturn(2159345.0);

    assert!(coordinates.longitude() > 6.0607944159 && coordinates.longitude() < 6.0607944161);
    assert!(coordinates.latitude() > -0.0336906977 && coordinates.latitude() < -0.0336906975);
    assert!(coordinates.distance() > 9.5927170 && coordinates.distance() < 9.5927178);

    let coordinates = vsop87b::saturn(2122820.0);

    assert!(coordinates.longitude() > 3.7760794189 && coordinates.longitude() < 3.7760794191);
    assert!(coordinates.latitude() > 0.0422300830 && coordinates.latitude() < 0.0422300832);
    assert!(coordinates.distance() > 9.8669935 && coordinates.distance() < 9.8669943);
}

#[test]
fn it_uranus() {
    let coordinates = vsop87b::uranus(2451545.0);

    assert!(coordinates.longitude() > 5.5225485296 && coordinates.longitude() < 5.5225485298);
    assert!(coordinates.latitude() > -0.0119527879 && coordinates.latitude() < -0.0119527877);
    assert!(coordinates.distance() > 19.9240475 && coordinates.distance() < 19.9240483);

    let coordinates = vsop87b::uranus(2415020.0);

    assert!(coordinates.longitude() > 4.3641525627 && coordinates.longitude() < 4.3641525629);
    assert!(coordinates.latitude() > 0.0009368609 && coordinates.latitude() < 0.0009368611);
    assert!(coordinates.distance() > 18.9927159 && coordinates.distance() < 18.9927167);

    let coordinates = vsop87b::uranus(2378495.0);

    assert!(coordinates.longitude() > 3.0875827850 && coordinates.longitude() < 3.0875827852);
    assert!(coordinates.latitude() > 0.0132269523 && coordinates.latitude() < 0.0132269525);
    assert!(coordinates.distance() > 18.2991148 && coordinates.distance() < 18.2991156);

    let coordinates = vsop87b::uranus(2341970.0);

    assert!(coordinates.longitude() > 1.7973185849 && coordinates.longitude() < 1.7973185851);
    assert!(coordinates.latitude() > 0.0066373710 && coordinates.latitude() < 0.0066373712);
    assert!(coordinates.distance() > 18.7966203 && coordinates.distance() < 18.7966211);

    let coordinates = vsop87b::uranus(2305445.0);

    assert!(coordinates.longitude() > 0.6197794564 && coordinates.longitude() < 0.6197794566);
    assert!(coordinates.latitude() > -0.0084149194 && coordinates.latitude() < -0.0084149192);
    assert!(coordinates.distance() > 19.7819882 && coordinates.distance() < 19.7819890);

    let coordinates = vsop87b::uranus(2268920.0);

    assert!(coordinates.longitude() > 5.8035494551 && coordinates.longitude() < 5.8035494553);
    assert!(coordinates.latitude() > -0.0133827061 && coordinates.latitude() < -0.0133827059);
    assert!(coordinates.distance() > 20.0300457 && coordinates.distance() < 20.0300465);

    let coordinates = vsop87b::uranus(2232395.0);

    assert!(coordinates.longitude() > 4.6715450661 && coordinates.longitude() < 4.6715450663);
    assert!(coordinates.latitude() > -0.0033027750 && coordinates.latitude() < -0.0033027748);
    assert!(coordinates.distance() > 19.2694309 && coordinates.distance() < 19.2694317);

    let coordinates = vsop87b::uranus(2195870.0);

    assert!(coordinates.longitude() > 3.4261485603 && coordinates.longitude() < 3.4261485605);
    assert!(coordinates.latitude() > 0.0115506262 && coordinates.latitude() < 0.0115506264);
    assert!(coordinates.distance() > 18.3948228 && coordinates.distance() < 18.3948236);

    let coordinates = vsop87b::uranus(2159345.0);

    assert!(coordinates.longitude() > 2.1281050049 && coordinates.longitude() < 2.1281050051);
    assert!(coordinates.latitude() > 0.0103036774 && coordinates.latitude() < 0.0103036776);
    assert!(coordinates.distance() > 18.5841494 && coordinates.distance() < 18.5841502);

    let coordinates = vsop87b::uranus(2122820.0);

    assert!(coordinates.longitude() > 0.9197293284 && coordinates.longitude() < 0.9197293286);
    assert!(coordinates.latitude() > -0.0048575383 && coordinates.latitude() < -0.0048575381);
    assert!(coordinates.distance() > 19.5612076 && coordinates.distance() < 19.5612084);
}

#[test]
fn it_neptune() {
    let coordinates = vsop87b::neptune(2451545.0);

    assert!(coordinates.longitude() > 5.3045629283 && coordinates.longitude() < 5.3045629285);
    assert!(coordinates.latitude() > 0.0042236789 && coordinates.latitude() < 0.0042236791);
    assert!(coordinates.distance() > 30.1205325 && coordinates.distance() < 30.1205333);

    let coordinates = vsop87b::neptune(2415020.0);

    assert!(coordinates.longitude() > 1.5199957207 && coordinates.longitude() < 1.5199957209);
    assert!(coordinates.latitude() > -0.0217331274 && coordinates.latitude() < -0.0217331272);
    assert!(coordinates.distance() > 29.8710341 && coordinates.distance() < 29.8710349);

    let coordinates = vsop87b::neptune(2378495.0);

    assert!(coordinates.longitude() > 3.9778043126 && coordinates.longitude() < 3.9778043128);
    assert!(coordinates.latitude() > 0.0307068992 && coordinates.latitude() < 0.0307068994);
    assert!(coordinates.distance() > 30.3209187 && coordinates.distance() < 30.3209195);

    let coordinates = vsop87b::neptune(2341970.0);

    assert!(coordinates.longitude() > 0.1546340454 && coordinates.longitude() < 0.1546340456);
    assert!(coordinates.latitude() > -0.0259181078 && coordinates.latitude() < -0.0259181076);
    assert!(coordinates.distance() > 29.8685857 && coordinates.distance() < 29.8685865);

    let coordinates = vsop87b::neptune(2305445.0);

    assert!(coordinates.longitude() > 2.6511574699 && coordinates.longitude() < 2.6511574701);
    assert!(coordinates.latitude() > 0.0106082424 && coordinates.latitude() < 0.0106082426);
    assert!(coordinates.distance() > 30.1360155 && coordinates.distance() < 30.1360163);

    let coordinates = vsop87b::neptune(2268920.0);

    assert!(coordinates.longitude() > 5.0896381604 && coordinates.longitude() < 5.0896381606);
    assert!(coordinates.latitude() > 0.0106592137 && coordinates.latitude() < 0.0106592139);
    assert!(coordinates.distance() > 30.1785346 && coordinates.distance() < 30.1785354);

    let coordinates = vsop87b::neptune(2232395.0);

    assert!(coordinates.longitude() > 1.2984703831 && coordinates.longitude() < 1.2984703833);
    assert!(coordinates.latitude() > -0.0260115822 && coordinates.latitude() < -0.0260115820);
    assert!(coordinates.distance() > 29.8326051 && coordinates.distance() < 29.8326059);

    let coordinates = vsop87b::neptune(2195870.0);

    assert!(coordinates.longitude() > 3.7635416327 && coordinates.longitude() < 3.7635416329);
    assert!(coordinates.latitude() > 0.0306777429 && coordinates.latitude() < 0.0306777431);
    assert!(coordinates.distance() > 30.3109111 && coordinates.distance() < 30.3109119);

    let coordinates = vsop87b::neptune(2159345.0);

    assert!(coordinates.longitude() > 6.2151087390 && coordinates.longitude() < 6.2151087392);
    assert!(coordinates.latitude() > -0.0215395778 && coordinates.latitude() < -0.0215395776);
    assert!(coordinates.distance() > 29.9065503 && coordinates.distance() < 29.9065511);

    let coordinates = vsop87b::neptune(2122820.0);

    assert!(coordinates.longitude() > 2.4315044301 && coordinates.longitude() < 2.4315044303);
    assert!(coordinates.latitude() > 0.0040125141 && coordinates.latitude() < 0.0040125143);
    assert!(coordinates.distance() > 30.0653691 && coordinates.distance() < 30.0653699);
}
