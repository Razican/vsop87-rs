extern crate vsop87;
use vsop87::*;

#[test]
fn it_mercury() {
    let coordinates = vsop87a::mercury(2451545.0);

    assert!(coordinates.x > -0.1300934116 && coordinates.x < -0.1300934114);
    assert!(coordinates.y > -0.4472876717 && coordinates.y < -0.4472876715);
    assert!(coordinates.z > -0.0246009 && coordinates.z < -0.0245959);

    let coordinates = vsop87a::mercury(2415020.0);

    assert!(coordinates.x > -0.3897246932 && coordinates.x < -0.3897246930);
    assert!(coordinates.y > -0.1502242200 && coordinates.y < -0.1502242198);
    assert!(coordinates.z > 0.0236174 && coordinates.z < 0.0236224);

    let coordinates = vsop87a::mercury(2378495.0);

    assert!(coordinates.x > -0.1683565238 && coordinates.x < -0.1683565236);
    assert!(coordinates.y > 0.2735108156 && coordinates.y < 0.2735108158);
    assert!(coordinates.z > 0.0378079 && coordinates.z < 0.0378129);

    let coordinates = vsop87a::mercury(2341970.0);

    assert!(coordinates.x > 0.3256720359 && coordinates.x < 0.3256720361);
    assert!(coordinates.y > 0.0880865801 && coordinates.y < 0.0880865803);
    assert!(coordinates.z > -0.0229845 && coordinates.z < -0.0229795);

    let coordinates = vsop87a::mercury(2305445.0);

    assert!(coordinates.x > 0.2314047966 && coordinates.x < 0.2314047968);
    assert!(coordinates.y > -0.3620120987 && coordinates.y < -0.3620120985);
    assert!(coordinates.z > -0.0508613 && coordinates.z < -0.0508563);

    let coordinates = vsop87a::mercury(2268920.0);

    assert!(coordinates.x > -0.1495554399 && coordinates.x < -0.1495554397);
    assert!(coordinates.y > -0.4409710105 && coordinates.y < -0.4409710103);
    assert!(coordinates.z > -0.0218152 && coordinates.z < -0.0218102);

    let coordinates = vsop87a::mercury(2232395.0);

    assert!(coordinates.x > -0.3938651888 && coordinates.x < -0.3938651886);
    assert!(coordinates.y > -0.1288399756 && coordinates.y < -0.1288399754);
    assert!(coordinates.z > 0.0263319 && coordinates.z < 0.0263369);

    let coordinates = vsop87a::mercury(2195870.0);

    assert!(coordinates.x > -0.1454241101 && coordinates.x < -0.1454241099);
    assert!(coordinates.y > 0.2837569446 && coordinates.y < 0.2837569448);
    assert!(coordinates.z > 0.0365234 && coordinates.z < 0.0365284);

    let coordinates = vsop87a::mercury(2159345.0);

    assert!(coordinates.x > 0.3340760580 && coordinates.x < 0.3340760582);
    assert!(coordinates.y > 0.0655125083 && coordinates.y < 0.0655125085);
    assert!(coordinates.z > -0.0260687 && coordinates.z < -0.0260637);

    let coordinates = vsop87a::mercury(2122820.0);

    assert!(coordinates.x > 0.2146329138 && coordinates.x < 0.2146329140);
    assert!(coordinates.y > -0.3752296251 && coordinates.y < -0.3752296249);
    assert!(coordinates.z > -0.0504008 && coordinates.z < -0.0503958);
}

#[test]
fn it_venus() {
    let coordinates = vsop87a::venus(2451545.0);

    assert!(coordinates.x > -0.7183022798 && coordinates.x < -0.7183022796);
    assert!(coordinates.y > -0.0326546018 && coordinates.y < -0.0326546016);
    assert!(coordinates.z > 0.0410118 && coordinates.z < 0.0410168);

    let coordinates = vsop87a::venus(2415020.0);

    assert!(coordinates.x > 0.6971428330 && coordinates.x < 0.6971428332);
    assert!(coordinates.y > -0.2033631152 && coordinates.y < -0.2033631150);
    assert!(coordinates.z > -0.0430226 && coordinates.z < -0.0430176);

    let coordinates = vsop87a::venus(2378495.0);

    assert!(coordinates.x > -0.5983535209 && coordinates.x < -0.5983535207);
    assert!(coordinates.y > 0.3958502155 && coordinates.y < 0.3958502157);
    assert!(coordinates.z > 0.0398213 && coordinates.z < 0.0398263);

    let coordinates = vsop87a::venus(2341970.0);

    assert!(coordinates.x > 0.4531193264 && coordinates.x < 0.4531193266);
    assert!(coordinates.y > -0.5692420970 && coordinates.y < -0.5692420968);
    assert!(coordinates.z > -0.0335668 && coordinates.z < -0.0335618);

    let coordinates = vsop87a::venus(2305445.0);

    assert!(coordinates.x > -0.2501974250 && coordinates.x < -0.2501974248);
    assert!(coordinates.y > 0.6732855398 && coordinates.y < 0.6732855400);
    assert!(coordinates.z > 0.0229690 && coordinates.z < 0.0229740);

    let coordinates = vsop87a::venus(2268920.0);

    assert!(coordinates.x > 0.0428334457 && coordinates.x < 0.0428334459);
    assert!(coordinates.y > -0.7259844931 && coordinates.y < -0.7259844929);
    assert!(coordinates.z > -0.0114050 && coordinates.z < -0.0114000);

    let coordinates = vsop87a::venus(2232395.0);

    assert!(coordinates.x > 0.1935421815 && coordinates.x < 0.1935421817);
    assert!(coordinates.y > 0.6940567995 && coordinates.y < 0.6940567997);
    assert!(coordinates.z > -0.0029328 && coordinates.z < -0.0029278);

    let coordinates = vsop87a::venus(2195870.0);

    assert!(coordinates.x > -0.3830059586 && coordinates.x < -0.3830059584);
    assert!(coordinates.y > -0.6150875571 && coordinates.y < -0.6150875569);
    assert!(coordinates.z > 0.0150895 && coordinates.z < 0.0150945);

    let coordinates = vsop87a::venus(2159345.0);

    assert!(coordinates.x > 0.5643550616 && coordinates.x < 0.5643550618);
    assert!(coordinates.y > 0.4519394441 && coordinates.y < 0.4519394443);
    assert!(coordinates.z > -0.0277269 && coordinates.z < -0.0277219);

    let coordinates = vsop87a::venus(2122820.0);

    assert!(coordinates.x > -0.6660158466 && coordinates.x < -0.6660158464);
    assert!(coordinates.y > -0.2753592312 && coordinates.y < -0.2753592310);
    assert!(coordinates.z > 0.0357849 && coordinates.z < 0.0357899);
}

#[test]
fn it_earth() {
    let coordinates = vsop87a::earth(2451545.0);

    assert!(coordinates.x > -0.1771354587 && coordinates.x < -0.1771354585);
    assert!(coordinates.y > 0.9672416236 && coordinates.y < 0.9672416238);
    assert!(coordinates.z > -0.0000064 && coordinates.z < -0.0000014);

    let coordinates = vsop87a::earth(2415020.0);

    assert!(coordinates.x > -0.1883079650 && coordinates.x < -0.1883079648);
    assert!(coordinates.y > 0.9650688843 && coordinates.y < 0.9650688845);
    assert!(coordinates.z > 0.0002125 && coordinates.z < 0.0002175);

    let coordinates = vsop87a::earth(2378495.0);

    assert!(coordinates.x > -0.1993918003 && coordinates.x < -0.1993918001);
    assert!(coordinates.y > 0.9627974367 && coordinates.y < 0.9627974369);
    assert!(coordinates.z > 0.0004283 && coordinates.z < 0.0004333);

    let coordinates = vsop87a::earth(2341970.0);

    assert!(coordinates.x > -0.2104654653 && coordinates.x < -0.2104654651);
    assert!(coordinates.y > 0.9603579953 && coordinates.y < 0.9603579955);
    assert!(coordinates.z > 0.0006448 && coordinates.z < 0.0006498);

    let coordinates = vsop87a::earth(2305445.0);

    assert!(coordinates.x > -0.2214982929 && coordinates.x < -0.2214982927);
    assert!(coordinates.y > 0.9578483180 && coordinates.y < 0.9578483182);
    assert!(coordinates.z > 0.0008543 && coordinates.z < 0.0008593);

    let coordinates = vsop87a::earth(2268920.0);

    assert!(coordinates.x > -0.2324780154 && coordinates.x < -0.2324780152);
    assert!(coordinates.y > 0.9551975792 && coordinates.y < 0.9551975794);
    assert!(coordinates.z > 0.0010668 && coordinates.z < 0.0010718);

    let coordinates = vsop87a::earth(2232395.0);

    assert!(coordinates.x > -0.2435134344 && coordinates.x < -0.2435134342);
    assert!(coordinates.y > 0.9524373310 && coordinates.y < 0.9524373312);
    assert!(coordinates.z > 0.0012846 && coordinates.z < 0.0012896);

    let coordinates = vsop87a::earth(2195870.0);

    assert!(coordinates.x > -0.2544603372 && coordinates.x < -0.2544603370);
    assert!(coordinates.y > 0.9495904256 && coordinates.y < 0.9495904258);
    assert!(coordinates.z > 0.0014937 && coordinates.z < 0.0014987);

    let coordinates = vsop87a::earth(2159345.0);

    assert!(coordinates.x > -0.2654547157 && coordinates.x < -0.2654547155);
    assert!(coordinates.y > 0.9465233601 && coordinates.y < 0.9465233603);
    assert!(coordinates.z > 0.0017013 && coordinates.z < 0.0017063);

    let coordinates = vsop87a::earth(2122820.0);

    assert!(coordinates.x > -0.2763146785 && coordinates.x < -0.2763146783);
    assert!(coordinates.y > 0.9433985306 && coordinates.y < 0.9433985308);
    assert!(coordinates.z > 0.0019090 && coordinates.z < 0.0019140);
}

#[test]
fn it_mars() {
    let coordinates = vsop87a::mars(2451545.0);

    assert!(coordinates.x > 1.3907159263 && coordinates.x < 1.3907159265);
    assert!(coordinates.y > -0.0134157044 && coordinates.y < -0.0134157042);
    assert!(coordinates.z > -0.0344703 && coordinates.z < -0.0344653);

    let coordinates = vsop87a::mars(2415020.0);

    assert!(coordinates.x > 0.4284332473 && coordinates.x < 0.4284332475);
    assert!(coordinates.y > -1.3552354251 && coordinates.y < -1.3552354249);
    assert!(coordinates.z > -0.0389675 && coordinates.z < -0.0389625);

    let coordinates = vsop87a::mars(2378495.0);

    assert!(coordinates.x > -1.1119219622 && coordinates.x < -1.1119219620);
    assert!(coordinates.y > -1.0963263015 && coordinates.y < -1.0963263013);
    assert!(coordinates.z > 0.0049184 && coordinates.z < 0.0049234);

    let coordinates = vsop87a::mars(2341970.0);

    assert!(coordinates.x > -1.6387489525 && coordinates.x < -1.6387489523);
    assert!(coordinates.y > 0.2507105241 && coordinates.y < 0.2507105243);
    assert!(coordinates.z > 0.0465581 && coordinates.z < 0.0465631);

    let coordinates = vsop87a::mars(2305445.0);

    assert!(coordinates.x > -0.8307668242 && coordinates.x < -0.8307668240);
    assert!(coordinates.y > 1.4098595096 && coordinates.y < 1.4098595098);
    assert!(coordinates.z > 0.0504511 && coordinates.z < 0.0504561);

    let coordinates = vsop87a::mars(2268920.0);

    assert!(coordinates.x > 0.6495258849 && coordinates.x < 0.6495258851);
    assert!(coordinates.y > 1.3657302244 && coordinates.y < 1.3657302246);
    assert!(coordinates.z > 0.0116897 && coordinates.z < 0.0116947);

    let coordinates = vsop87a::mars(2232395.0);

    assert!(coordinates.x > 1.3910394545 && coordinates.x < 1.3910394547);
    assert!(coordinates.y > -0.0543839268 && coordinates.y < -0.0543839266);
    assert!(coordinates.z > -0.0371038 && coordinates.z < -0.0370988);

    let coordinates = vsop87a::mars(2195870.0);

    assert!(coordinates.x > 0.3890073908 && coordinates.x < 0.3890073910);
    assert!(coordinates.y > -1.3660431024 && coordinates.y < -1.3660431022);
    assert!(coordinates.z > -0.0383834 && coordinates.z < -0.0383784);

    let coordinates = vsop87a::mars(2159345.0);

    assert!(coordinates.x > -1.1440917097 && coordinates.x < -1.1440917095);
    assert!(coordinates.y > -1.0595533317 && coordinates.y < -1.0595533315);
    assert!(coordinates.z > 0.0082156 && coordinates.z < 0.0082206);

    let coordinates = vsop87a::mars(2122820.0);

    assert!(coordinates.x > -1.6278485158 && coordinates.x < -1.6278485156);
    assert!(coordinates.y > 0.3060194813 && coordinates.y < 0.3060194815);
    assert!(coordinates.z > 0.0494191 && coordinates.z < 0.0494241);
}

#[test]
fn it_jupiter() {
    let coordinates = vsop87a::jupiter(2451545.0);

    assert!(coordinates.x > 4.0011740267 && coordinates.x < 4.0011740269);
    assert!(coordinates.y > 2.9385810076 && coordinates.y < 2.9385810078);
    assert!(coordinates.z > -0.1017863 && coordinates.z < -0.1017813);

    let coordinates = vsop87a::jupiter(2415020.0);

    assert!(coordinates.x > -3.0191224351 && coordinates.x < -3.0191224349);
    assert!(coordinates.y > -4.4582563706 && coordinates.y < -4.4582563704);
    assert!(coordinates.z > 0.0858617 && coordinates.z < 0.0858667);

    let coordinates = vsop87a::jupiter(2378495.0);

    assert!(coordinates.x > -0.0180390005 && coordinates.x < -0.0180390003);
    assert!(coordinates.y > 5.1317748838 && coordinates.y < 5.1317748840);
    assert!(coordinates.z > -0.0200473 && coordinates.z < -0.0200423);

    let coordinates = vsop87a::jupiter(2341970.0);

    assert!(coordinates.x > 1.2817318352 && coordinates.x < 1.2817318354);
    assert!(coordinates.y > -5.0280079875 && coordinates.y < -5.0280079873);
    assert!(coordinates.z > -0.0091277 && coordinates.z < -0.0091227);

    let coordinates = vsop87a::jupiter(2305445.0);

    assert!(coordinates.x > -4.0547959776 && coordinates.x < -4.0547959774);
    assert!(coordinates.y > 3.4799857071 && coordinates.y < 3.4799857073);
    assert!(coordinates.z > 0.0779935 && coordinates.z < 0.0779985);

    let coordinates = vsop87a::jupiter(2268920.0);

    assert!(coordinates.x > 4.5891471726 && coordinates.x < 4.5891471728);
    assert!(coordinates.y > -1.9870837932 && coordinates.y < -1.9870837930);
    assert!(coordinates.z > -0.0961117 && coordinates.z < -0.0961067);

    let coordinates = vsop87a::jupiter(2232395.0);

    assert!(coordinates.x > -5.4239396006 && coordinates.x < -5.4239396004);
    assert!(coordinates.y > -0.5085487292 && coordinates.y < -0.5085487290);
    assert!(coordinates.z > 0.1247735 && coordinates.z < 0.1247785);

    let coordinates = vsop87a::jupiter(2195870.0);

    assert!(coordinates.x > 4.2423286339 && coordinates.x < 4.2423286341);
    assert!(coordinates.y > 2.5898433578 && coordinates.y < 2.5898433580);
    assert!(coordinates.z > -0.1060332 && coordinates.z < -0.1060282);

    let coordinates = vsop87a::jupiter(2159345.0);

    assert!(coordinates.x > -3.3554806096 && coordinates.x < -3.3554806094);
    assert!(coordinates.y > -4.2166702225 && coordinates.y < -4.2166702223);
    assert!(coordinates.z > 0.0919393 && coordinates.z < 0.0919443);

    let coordinates = vsop87a::jupiter(2122820.0);

    assert!(coordinates.x > 0.4207861893 && coordinates.x < 0.4207861895);
    assert!(coordinates.y > 5.1019591309 && coordinates.y < 5.1019591311);
    assert!(coordinates.z > -0.0280113 && coordinates.z < -0.0280063);
}

#[test]
fn it_saturn() {
    let coordinates = vsop87a::saturn(2451545.0);

    assert!(coordinates.x > 6.4064068572 && coordinates.x < 6.4064068574);
    assert!(coordinates.y > 6.5699929448 && coordinates.y < 6.5699929450);
    assert!(coordinates.z > -0.3690793 && coordinates.z < -0.3690743);

    let coordinates = vsop87a::saturn(2415020.0);

    assert!(coordinates.x > -0.3695973751 && coordinates.x < -0.3695973749);
    assert!(coordinates.y > -10.0582398189 && coordinates.y < -10.0582398187);
    assert!(coordinates.z > 0.1916829 && coordinates.z < 0.1916879);

    let coordinates = vsop87a::saturn(2378495.0);

    assert!(coordinates.x > -5.6790910871 && coordinates.x < -5.6790910869);
    assert!(coordinates.y > 7.1152478119 && coordinates.y < 7.1152478121);
    assert!(coordinates.z > 0.0978496 && coordinates.z < 0.0978546);

    let coordinates = vsop87a::saturn(2341970.0);

    assert!(coordinates.x > 8.9934758991 && coordinates.x < 8.9934758993);
    assert!(coordinates.y > -3.7883225438 && coordinates.y < -3.7883225436);
    assert!(coordinates.z > -0.2866414 && coordinates.z < -0.2866364);

    let coordinates = vsop87a::saturn(2305445.0);

    assert!(coordinates.x > -8.6570276347 && coordinates.x < -8.6570276345);
    assert!(coordinates.y > -4.4809792499 && coordinates.y < -4.4809792497);
    assert!(coordinates.z > 0.4216227 && coordinates.z < 0.4216277);

    let coordinates = vsop87a::saturn(2268920.0);

    assert!(coordinates.x > 5.0378574918 && coordinates.x < 5.0378574920);
    assert!(coordinates.y > 7.5310625789 && coordinates.y < 7.5310625791);
    assert!(coordinates.z > -0.3348906 && coordinates.z < -0.3348856);

    let coordinates = vsop87a::saturn(2232395.0);

    assert!(coordinates.x > 1.2601620698 && coordinates.x < 1.2601620700);
    assert!(coordinates.y > -10.0267935694 && coordinates.y < -10.0267935692);
    assert!(coordinates.z > 0.1347028 && coordinates.z < 0.1347078);

    let coordinates = vsop87a::saturn(2195870.0);

    assert!(coordinates.x > -7.1628125748 && coordinates.x < -7.1628125746);
    assert!(coordinates.y > 5.7482646990 && coordinates.y < 5.7482646992);
    assert!(coordinates.z > 0.1724937 && coordinates.z < 0.1724987);

    let coordinates = vsop87a::saturn(2159345.0);

    assert!(coordinates.x > 9.3511669241 && coordinates.x < 9.3511669243);
    assert!(coordinates.y > -2.1145906250 && coordinates.y < -2.1145906248);
    assert!(coordinates.z > -0.3231267 && coordinates.z < -0.3231217);

    let coordinates = vsop87a::saturn(2122820.0);

    assert!(coordinates.x > -7.9395559174 && coordinates.x < -7.9395559172);
    assert!(coordinates.y > -5.8435867017 && coordinates.y < -5.8435867015);
    assert!(coordinates.z > 0.4165577 && coordinates.z < 0.4165627);
}

#[test]
fn it_uranus() {
    let coordinates = vsop87a::uranus(2451545.0);

    assert!(coordinates.x > 14.4318934158 && coordinates.x < 14.4318934160);
    assert!(coordinates.y > -13.7343162528 && coordinates.y < -13.7343162526);
    assert!(coordinates.z > -0.2381447 && coordinates.z < -0.2381397);

    let coordinates = vsop87a::uranus(2415020.0);

    assert!(coordinates.x > -6.4810833338 && coordinates.x < -6.4810833336);
    assert!(coordinates.y > -17.8526893407 && coordinates.y < -17.8526893405);
    assert!(coordinates.z > 0.0177910 && coordinates.z < 0.0177960);

    let coordinates = vsop87a::uranus(2378495.0);

    assert!(coordinates.x > -18.2708335179 && coordinates.x < -18.2708335177);
    assert!(coordinates.y > 0.9877655714 && coordinates.y < 0.9877655716);
    assert!(coordinates.z > 0.2420319 && coordinates.z < 0.2420369);

    let coordinates = vsop87a::uranus(2341970.0);

    assert!(coordinates.x > -4.2214391937 && coordinates.x < -4.2214391935);
    assert!(coordinates.y > 18.3160266383 && coordinates.y < 18.3160266385);
    assert!(coordinates.z > 0.1247568 && coordinates.z < 0.1247618);

    let coordinates = vsop87a::uranus(2305445.0);

    assert!(coordinates.x > 16.1020987625 && coordinates.x < 16.1020987627);
    assert!(coordinates.y > 11.4900726863 && coordinates.y < 11.4900726865);
    assert!(coordinates.z > -0.1664644 && coordinates.z < -0.1664594);

    let coordinates = vsop87a::uranus(2268920.0);

    assert!(coordinates.x > 17.7683247786 && coordinates.x < 17.7683247788);
    assert!(coordinates.y > -9.2421595877 && coordinates.y < -9.2421595875);
    assert!(coordinates.z > -0.2680507 && coordinates.z < -0.2680457);

    let coordinates = vsop87a::uranus(2232395.0);

    assert!(coordinates.x > -0.7868164613 && coordinates.x < -0.7868164611);
    assert!(coordinates.y > -19.2532559479 && coordinates.y < -19.2532559477);
    assert!(coordinates.z > -0.0636449 && coordinates.z < -0.0636399);

    let coordinates = vsop87a::uranus(2195870.0);

    assert!(coordinates.x > -17.6539243376 && coordinates.x < -17.6539243374);
    assert!(coordinates.y > -5.1636568777 && coordinates.y < -5.1636568775);
    assert!(coordinates.z > 0.2124644 && coordinates.z < 0.2124694);

    let coordinates = vsop87a::uranus(2159345.0);

    assert!(coordinates.x > -9.8287104598 && coordinates.x < -9.8287104596);
    assert!(coordinates.y > 15.7711888604 && coordinates.y < 15.7711888606);
    assert!(coordinates.z > 0.1914792 && coordinates.z < 0.1914842);

    let coordinates = vsop87a::uranus(2122820.0);

    assert!(coordinates.x > 11.8546461038 && coordinates.x < 11.8546461040);
    assert!(coordinates.y > 15.5595370552 && coordinates.y < 15.5595370554);
    assert!(coordinates.z > -0.0950214 && coordinates.z < -0.0950164);
}

#[test]
fn it_neptune() {
    let coordinates = vsop87a::neptune(2451545.0);

    assert!(coordinates.x > 16.8121116575 && coordinates.x < 16.8121116577);
    assert!(coordinates.y > -24.9916630909 && coordinates.y < -24.9916630907);
    assert!(coordinates.z > 0.1272165 && coordinates.z < 0.1272215);

    let coordinates = vsop87a::neptune(2415020.0);

    assert!(coordinates.x > 1.5164557466 && coordinates.x < 1.5164557468);
    assert!(coordinates.y > 29.8254538900 && coordinates.y < 29.8254538902);
    assert!(coordinates.z > -0.6491425 && coordinates.z < -0.6491375);

    let coordinates = vsop87a::neptune(2378495.0);

    assert!(coordinates.x > -20.3138943579 && coordinates.x < -20.3138943577);
    assert!(coordinates.y > -22.4908255797 && coordinates.y < -22.4908255795);
    assert!(coordinates.z > 0.9309127 && coordinates.z < 0.9309177);

    let coordinates = vsop87a::neptune(2341970.0);

    assert!(coordinates.x > 29.5022811949 && coordinates.x < 29.5022811951);
    assert!(coordinates.y > 4.5987701113 && coordinates.y < 4.5987701115);
    assert!(coordinates.z > -0.7740532 && coordinates.z < -0.7740482);

    let coordinates = vsop87a::neptune(2305445.0);

    assert!(coordinates.x > -26.5823264273 && coordinates.x < -26.5823264271);
    assert!(coordinates.y > 14.1935610228 && coordinates.y < 14.1935610230);
    assert!(coordinates.z > 0.3196817 && coordinates.z < 0.3196867);

    let coordinates = vsop87a::neptune(2268920.0);

    assert!(coordinates.x > 11.1160686192 && coordinates.x < 11.1160686194);
    assert!(coordinates.y > -28.0548309590 && coordinates.y < -28.0548309588);
    assert!(coordinates.z > 0.3216707 && coordinates.z < 0.3216757);

    let coordinates = vsop87a::neptune(2232395.0);

    assert!(coordinates.x > 8.0214324005 && coordinates.x < 8.0214324007);
    assert!(coordinates.y > 28.7234916079 && coordinates.y < 28.7234916081);
    assert!(coordinates.z > -0.7759084 && coordinates.z < -0.7759034);

    let coordinates = vsop87a::neptune(2195870.0);

    assert!(coordinates.x > -24.6234347579 && coordinates.x < -24.6234347577);
    assert!(coordinates.y > -17.6514428047 && coordinates.y < -17.6514428045);
    assert!(coordinates.z > 0.9297219 && coordinates.z < 0.9297269);

    let coordinates = vsop87a::neptune(2159345.0);

    assert!(coordinates.x > 29.8303563035 && coordinates.x < 29.8303563037);
    assert!(coordinates.y > -2.0338910504 && coordinates.y < -2.0338910502);
    assert!(coordinates.z > -0.6441272 && coordinates.z < -0.6441222);

    let coordinates = vsop87a::neptune(2122820.0);

    assert!(coordinates.x > -22.7985170871 && coordinates.x < -22.7985170869);
    assert!(coordinates.y > 19.5994768857 && coordinates.y < 19.5994768859);
    assert!(coordinates.z > 0.1206349 && coordinates.z < 0.1206399);
}

#[test]
fn it_earth_moon() {
    let coordinates = vsop87a::earth_moon(2451545.0);

    assert!(coordinates.x > -0.1771591441 && coordinates.x < -0.1771591439);
    assert!(coordinates.y > 0.9672192890 && coordinates.y < 0.9672192892);
    assert!(coordinates.z > -0.0000035 && coordinates.z < 0.0000015);

    let coordinates = vsop87a::earth_moon(2415020.0);

    assert!(coordinates.x > -0.1883097014 && coordinates.x < -0.1883097012);
    assert!(coordinates.y > 0.9650388427 && coordinates.y < 0.9650388429);
    assert!(coordinates.z > 0.0002128 && coordinates.z < 0.0002178);

    let coordinates = vsop87a::earth_moon(2378495.0);

    assert!(coordinates.x > -0.1993643285 && coordinates.x < -0.1993643283);
    assert!(coordinates.y > 0.9627828194 && coordinates.y < 0.9627828196);
    assert!(coordinates.z > 0.0004258 && coordinates.z < 0.0004308);

    let coordinates = vsop87a::earth_moon(2341970.0);

    assert!(coordinates.x > -0.2104343222 && coordinates.x < -0.2104343220);
    assert!(coordinates.y > 0.9603642781 && coordinates.y < 0.9603642783);
    assert!(coordinates.z > 0.0006438 && coordinates.z < 0.0006488);

    let coordinates = vsop87a::earth_moon(2305445.0);

    assert!(coordinates.x > -0.2214911210 && coordinates.x < -0.2214911208);
    assert!(coordinates.y > 0.9578778166 && coordinates.y < 0.9578778168);
    assert!(coordinates.z > 0.0008565 && coordinates.z < 0.0008615);

    let coordinates = vsop87a::earth_moon(2268920.0);

    assert!(coordinates.x > -0.2324953838 && coordinates.x < -0.2324953836);
    assert!(coordinates.y > 0.9552252050 && coordinates.y < 0.9552252052);
    assert!(coordinates.z > 0.0010687 && coordinates.z < 0.0010737);

    let coordinates = vsop87a::earth_moon(2232395.0);

    assert!(coordinates.x > -0.2435434219 && coordinates.x < -0.2435434217);
    assert!(coordinates.y > 0.9524355202 && coordinates.y < 0.9524355204);
    assert!(coordinates.z > 0.0012830 && coordinates.z < 0.0012880);

    let coordinates = vsop87a::earth_moon(2195870.0);

    assert!(coordinates.x > -0.2544800657 && coordinates.x < -0.2544800655);
    assert!(coordinates.y > 0.9495642256 && coordinates.y < 0.9495642258);
    assert!(coordinates.z > 0.0014912 && coordinates.z < 0.0014962);

    let coordinates = vsop87a::earth_moon(2159345.0);

    assert!(coordinates.x > -0.2654471687 && coordinates.x < -0.2654471685);
    assert!(coordinates.y > 0.9464953235 && coordinates.y < 0.9464953237);
    assert!(coordinates.z > 0.0017023 && coordinates.z < 0.0017073);

    let coordinates = vsop87a::earth_moon(2122820.0);

    assert!(coordinates.x > -0.2762837552 && coordinates.x < -0.2762837550);
    assert!(coordinates.y > 0.9433889918 && coordinates.y < 0.9433889920);
    assert!(coordinates.z > 0.0019119 && coordinates.z < 0.0019169);
}
