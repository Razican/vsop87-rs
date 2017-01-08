extern crate vsop87;

#[test]
fn it_mercury() {
    let vsop87_elts = vsop87::mercury(2451545.0);

    assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
    assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
    assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
    assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
    assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
    assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);

    let vsop87_elts = vsop87::mercury(2415020.0);

    assert!(vsop87_elts.a > 0.3870977205 && vsop87_elts.a < 0.3870977207);
    assert!(vsop87_elts.l > 3.1341564064 && vsop87_elts.l < 3.1341564066);
    assert!(vsop87_elts.k > 0.0452159417 && vsop87_elts.k < 0.0452159419);
    assert!(vsop87_elts.h > 0.2005915793 && vsop87_elts.h < 0.2005915795);
    assert!(vsop87_elts.q > 0.0405500077 && vsop87_elts.q < 0.0405500079);
    assert!(vsop87_elts.p > 0.04576328 && vsop87_elts.p < 0.04576404);

    let vsop87_elts = vsop87::mercury(2378495.0);

    assert!(vsop87_elts.a > 0.3870988717 && vsop87_elts.a < 0.3870988719);
    assert!(vsop87_elts.l > 1.8657954072 && vsop87_elts.l < 1.8657954074);
    assert!(vsop87_elts.k > 0.0457588297 && vsop87_elts.k < 0.0457588299);
    assert!(vsop87_elts.h > 0.2004369023 && vsop87_elts.h < 0.2004369025);
    assert!(vsop87_elts.q > 0.0404841230 && vsop87_elts.q < 0.0404841232);
    assert!(vsop87_elts.p > 0.04589017 && vsop87_elts.p < 0.04589093);

    let vsop87_elts = vsop87::mercury(2341970.0);

    assert!(vsop87_elts.a > 0.3870981591 && vsop87_elts.a < 0.3870981593);
    assert!(vsop87_elts.l > 0.5973516889 && vsop87_elts.l < 0.5973516891);
    assert!(vsop87_elts.k > 0.0462989388 && vsop87_elts.k < 0.0462989390);
    assert!(vsop87_elts.h > 0.2002875698 && vsop87_elts.h < 0.2002875700);
    assert!(vsop87_elts.q > 0.0404178809 && vsop87_elts.q < 0.0404178811);
    assert!(vsop87_elts.p > 0.04601680 && vsop87_elts.p < 0.04601756);

    let vsop87_elts = vsop87::mercury(2305445.0);

    assert!(vsop87_elts.a > 0.3870993314 && vsop87_elts.a < 0.3870993316);
    assert!(vsop87_elts.l > 5.6121176788 && vsop87_elts.l < 5.6121176790);
    assert!(vsop87_elts.k > 0.0468597369 && vsop87_elts.k < 0.0468597371);
    assert!(vsop87_elts.h > 0.2001268145 && vsop87_elts.h < 0.2001268147);
    assert!(vsop87_elts.q > 0.0403519525 && vsop87_elts.q < 0.0403519527);
    assert!(vsop87_elts.p > 0.04614370 && vsop87_elts.p < 0.04614446);

    let vsop87_elts = vsop87::mercury(2268920.0);

    assert!(vsop87_elts.a > 0.3870987714 && vsop87_elts.a < 0.3870987716);
    assert!(vsop87_elts.l > 4.3437933703 && vsop87_elts.l < 4.3437933705);
    assert!(vsop87_elts.k > 0.0474175873 && vsop87_elts.k < 0.0474175875);
    assert!(vsop87_elts.h > 0.1999792559 && vsop87_elts.h < 0.1999792561);
    assert!(vsop87_elts.q > 0.0402856290 && vsop87_elts.q < 0.0402856292);
    assert!(vsop87_elts.p > 0.04627099 && vsop87_elts.p < 0.04627175);

    let vsop87_elts = vsop87::mercury(2232395.0);

    assert!(vsop87_elts.a > 0.3870987732 && vsop87_elts.a < 0.3870987734);
    assert!(vsop87_elts.l > 3.0753253704 && vsop87_elts.l < 3.0753253706);
    assert!(vsop87_elts.k > 0.0479739902 && vsop87_elts.k < 0.0479739904);
    assert!(vsop87_elts.h > 0.1998210216 && vsop87_elts.h < 0.1998210218);
    assert!(vsop87_elts.q > 0.0402194430 && vsop87_elts.q < 0.0402194432);
    assert!(vsop87_elts.p > 0.04639750 && vsop87_elts.p < 0.04639826);

    let vsop87_elts = vsop87::mercury(2195870.0);

    assert!(vsop87_elts.a > 0.3870975282 && vsop87_elts.a < 0.3870975284);
    assert!(vsop87_elts.l > 1.8068957209 && vsop87_elts.l < 1.8068957211);
    assert!(vsop87_elts.k > 0.0485220074 && vsop87_elts.k < 0.0485220076);
    assert!(vsop87_elts.h > 0.1996800562 && vsop87_elts.h < 0.1996800564);
    assert!(vsop87_elts.q > 0.0401524549 && vsop87_elts.q < 0.0401524551);
    assert!(vsop87_elts.p > 0.04652410 && vsop87_elts.p < 0.04652486);

    let vsop87_elts = vsop87::mercury(2159345.0);

    assert!(vsop87_elts.a > 0.3870977788 && vsop87_elts.a < 0.3870977790);
    assert!(vsop87_elts.l > 0.5385405029 && vsop87_elts.l < 0.5385405031);
    assert!(vsop87_elts.k > 0.0490792961 && vsop87_elts.k < 0.0490792963);
    assert!(vsop87_elts.h > 0.1995293399 && vsop87_elts.h < 0.1995293401);
    assert!(vsop87_elts.q > 0.0400854728 && vsop87_elts.q < 0.0400854730);
    assert!(vsop87_elts.p > 0.04665108 && vsop87_elts.p < 0.04665184);

    let vsop87_elts = vsop87::mercury(2122820.0);

    assert!(vsop87_elts.a > 0.3870983086 && vsop87_elts.a < 0.3870983088);
    assert!(vsop87_elts.l > 5.5532501233 && vsop87_elts.l < 5.5532501235);
    assert!(vsop87_elts.k > 0.0496101938 && vsop87_elts.k < 0.0496101940);
    assert!(vsop87_elts.h > 0.1993701566 && vsop87_elts.h < 0.1993701568);
    assert!(vsop87_elts.q > 0.0400177911 && vsop87_elts.q < 0.0400177913);
    assert!(vsop87_elts.p > 0.04677625 && vsop87_elts.p < 0.04677701);
}

#[test]
fn it_venus() {
    let vsop87_elts = vsop87::venus(2451545.0);

    assert!(vsop87_elts.a > 0.7233269303 && vsop87_elts.a < 0.7233269305);
    assert!(vsop87_elts.l > 3.1761350909 && vsop87_elts.l < 3.1761350911);
    assert!(vsop87_elts.k > -0.0045086078 && vsop87_elts.k < -0.0045086076);
    assert!(vsop87_elts.h > 0.0050312181 && vsop87_elts.h < 0.0050312183);
    assert!(vsop87_elts.q > 0.0068248057 && vsop87_elts.q < 0.0068248059);
    assert!(vsop87_elts.p > 0.02882177 && vsop87_elts.p < 0.02882253);

    let vsop87_elts = vsop87::venus(2415020.0);

    assert!(vsop87_elts.a > 0.7233254386 && vsop87_elts.a < 0.7233254388);
    assert!(vsop87_elts.l > 6.0067809875 && vsop87_elts.l < 6.0067809877);
    assert!(vsop87_elts.k > -0.0044945273 && vsop87_elts.k < -0.0044945271);
    assert!(vsop87_elts.h > 0.0051121732 && vsop87_elts.h < 0.0051121734);
    assert!(vsop87_elts.q > 0.0066855873 && vsop87_elts.q < 0.0066855875);
    assert!(vsop87_elts.p > 0.02886332 && vsop87_elts.p < 0.02886408);

    let vsop87_elts = vsop87::venus(2378495.0);

    assert!(vsop87_elts.a > 0.7233259623 && vsop87_elts.a < 0.7233259625);
    assert!(vsop87_elts.l > 2.5541924032 && vsop87_elts.l < 2.5541924034);
    assert!(vsop87_elts.k > -0.0045347114 && vsop87_elts.k < -0.0045347112);
    assert!(vsop87_elts.h > 0.0051153685 && vsop87_elts.h < 0.0051153687);
    assert!(vsop87_elts.q > 0.0065487272 && vsop87_elts.q < 0.0065487274);
    assert!(vsop87_elts.p > 0.02890118 && vsop87_elts.p < 0.02890194);

    let vsop87_elts = vsop87::venus(2341970.0);

    assert!(vsop87_elts.a > 0.7233336009 && vsop87_elts.a < 0.7233336011);
    assert!(vsop87_elts.l > 5.3848916849 && vsop87_elts.l < 5.3848916851);
    assert!(vsop87_elts.k > -0.0045714047 && vsop87_elts.k < -0.0045714045);
    assert!(vsop87_elts.h > 0.0052085665 && vsop87_elts.h < 0.0052085667);
    assert!(vsop87_elts.q > 0.0064078361 && vsop87_elts.q < 0.0064078363);
    assert!(vsop87_elts.p > 0.02893776 && vsop87_elts.p < 0.02893852);

    let vsop87_elts = vsop87::venus(2305445.0);

    assert!(vsop87_elts.a > 0.7233444127 && vsop87_elts.a < 0.7233444129);
    assert!(vsop87_elts.l > 1.9323601819 && vsop87_elts.l < 1.9323601821);
    assert!(vsop87_elts.k > -0.0046106822 && vsop87_elts.k < -0.0046106820);
    assert!(vsop87_elts.h > 0.0052198094 && vsop87_elts.h < 0.0052198096);
    assert!(vsop87_elts.q > 0.0062695295 && vsop87_elts.q < 0.0062695297);
    assert!(vsop87_elts.p > 0.02897438 && vsop87_elts.p < 0.02897514);

    let vsop87_elts = vsop87::venus(2268920.0);

    assert!(vsop87_elts.a > 0.7233295420 && vsop87_elts.a < 0.7233295422);
    assert!(vsop87_elts.l > 4.7630365986 && vsop87_elts.l < 4.7630365988);
    assert!(vsop87_elts.k > -0.0046705687 && vsop87_elts.k < -0.0046705685);
    assert!(vsop87_elts.h > 0.0052905837 && vsop87_elts.h < 0.0052905839);
    assert!(vsop87_elts.q > 0.0061312555 && vsop87_elts.q < 0.0061312557);
    assert!(vsop87_elts.p > 0.02900830 && vsop87_elts.p < 0.02900906);

    let vsop87_elts = vsop87::venus(2232395.0);

    assert!(vsop87_elts.a > 0.7233288476 && vsop87_elts.a < 0.7233288478);
    assert!(vsop87_elts.l > 1.3105445954 && vsop87_elts.l < 1.3105445956);
    assert!(vsop87_elts.k > -0.0046816593 && vsop87_elts.k < -0.0046816591);
    assert!(vsop87_elts.h > 0.0052685213 && vsop87_elts.h < 0.0052685215);
    assert!(vsop87_elts.q > 0.0059904847 && vsop87_elts.q < 0.0059904849);
    assert!(vsop87_elts.p > 0.02904208 && vsop87_elts.p < 0.02904284);

    let vsop87_elts = vsop87::venus(2195870.0);

    assert!(vsop87_elts.a > 0.7233301131 && vsop87_elts.a < 0.7233301133);
    assert!(vsop87_elts.l > 4.1411623838 && vsop87_elts.l < 4.1411623840);
    assert!(vsop87_elts.k > -0.0047512384 && vsop87_elts.k < -0.0047512382);
    assert!(vsop87_elts.h > 0.0053213978 && vsop87_elts.h < 0.0053213980);
    assert!(vsop87_elts.q > 0.0058529531 && vsop87_elts.q < 0.0058529533);
    assert!(vsop87_elts.p > 0.02907519 && vsop87_elts.p < 0.02907595);

    let vsop87_elts = vsop87::venus(2159345.0);

    assert!(vsop87_elts.a > 0.7233280697 && vsop87_elts.a < 0.7233280699);
    assert!(vsop87_elts.l > 0.6886289924 && vsop87_elts.l < 0.6886289926);
    assert!(vsop87_elts.k > -0.0047241640 && vsop87_elts.k < -0.0047241638);
    assert!(vsop87_elts.h > 0.0053523285 && vsop87_elts.h < 0.0053523287);
    assert!(vsop87_elts.q > 0.0057132256 && vsop87_elts.q < 0.0057132258);
    assert!(vsop87_elts.p > 0.02910527 && vsop87_elts.p < 0.02910603);

    let vsop87_elts = vsop87::venus(2122820.0);

    assert!(vsop87_elts.a > 0.7233247250 && vsop87_elts.a < 0.7233247252);
    assert!(vsop87_elts.l > 3.5192700748 && vsop87_elts.l < 3.5192700750);
    assert!(vsop87_elts.k > -0.0047739163 && vsop87_elts.k < -0.0047739161);
    assert!(vsop87_elts.h > 0.0053755161 && vsop87_elts.h < 0.0053755163);
    assert!(vsop87_elts.q > 0.0055732703 && vsop87_elts.q < 0.0055732705);
    assert!(vsop87_elts.p > 0.02913516 && vsop87_elts.p < 0.02913592);
}

#[test]
fn it_mars() {
    let vsop87_elts = vsop87::mars(2451545.0);

    assert!(vsop87_elts.a > 1.5236789886 && vsop87_elts.a < 1.5236789888);
    assert!(vsop87_elts.l > 6.2038757098 && vsop87_elts.l < 6.2038757100);
    assert!(vsop87_elts.k > 0.0853133077 && vsop87_elts.k < 0.0853133079);
    assert!(vsop87_elts.h > -0.0378067118 && vsop87_elts.h < -0.0378067116);
    assert!(vsop87_elts.q > 0.0104705228 && vsop87_elts.q < 0.0104705230);
    assert!(vsop87_elts.p > 0.01228588 && vsop87_elts.p < 0.01228664);

    let vsop87_elts = vsop87::mars(2415020.0);

    assert!(vsop87_elts.a > 1.5236463471 && vsop87_elts.a < 1.5236463473);
    assert!(vsop87_elts.l > 5.1511909416 && vsop87_elts.l < 5.1511909418);
    assert!(vsop87_elts.k > 0.0849023066 && vsop87_elts.k < 0.0849023068);
    assert!(vsop87_elts.h > -0.0384449186 && vsop87_elts.h < -0.0384449184);
    assert!(vsop87_elts.q > 0.0104523542 && vsop87_elts.q < 0.0104523544);
    assert!(vsop87_elts.p > 0.01239228 && vsop87_elts.p < 0.01239304);

    let vsop87_elts = vsop87::mars(2378495.0);

    assert!(vsop87_elts.a > 1.5236769794 && vsop87_elts.a < 1.5236769796);
    assert!(vsop87_elts.l > 4.0987790796 && vsop87_elts.l < 4.0987790798);
    assert!(vsop87_elts.k > 0.0846457411 && vsop87_elts.k < 0.0846457413);
    assert!(vsop87_elts.h > -0.0392454968 && vsop87_elts.h < -0.0392454966);
    assert!(vsop87_elts.q > 0.0104339357 && vsop87_elts.q < 0.0104339359);
    assert!(vsop87_elts.p > 0.01249909 && vsop87_elts.p < 0.01249985);

    let vsop87_elts = vsop87::mars(2341970.0);

    assert!(vsop87_elts.a > 1.5236301763 && vsop87_elts.a < 1.5236301765);
    assert!(vsop87_elts.l > 3.0462792684 && vsop87_elts.l < 3.0462792686);
    assert!(vsop87_elts.k > 0.0842278892 && vsop87_elts.k < 0.0842278894);
    assert!(vsop87_elts.h > -0.0396982630 && vsop87_elts.h < -0.0396982628);
    assert!(vsop87_elts.q > 0.0104148074 && vsop87_elts.q < 0.0104148076);
    assert!(vsop87_elts.p > 0.01260671 && vsop87_elts.p < 0.01260747);

    let vsop87_elts = vsop87::mars(2305445.0);

    assert!(vsop87_elts.a > 1.5237744504 && vsop87_elts.a < 1.5237744506);
    assert!(vsop87_elts.l > 1.9938502930 && vsop87_elts.l < 1.9938502932);
    assert!(vsop87_elts.k > 0.0837767412 && vsop87_elts.k < 0.0837767414);
    assert!(vsop87_elts.h > -0.0403804192 && vsop87_elts.h < -0.0403804190);
    assert!(vsop87_elts.q > 0.0103948926 && vsop87_elts.q < 0.0103948928);
    assert!(vsop87_elts.p > 0.01271228 && vsop87_elts.p < 0.01271304);

    let vsop87_elts = vsop87::mars(2268920.0);

    assert!(vsop87_elts.a > 1.5236102236 && vsop87_elts.a < 1.5236102238);
    assert!(vsop87_elts.l > 0.9412449946 && vsop87_elts.l < 0.9412449948);
    assert!(vsop87_elts.k > 0.0833757043 && vsop87_elts.k < 0.0833757045);
    assert!(vsop87_elts.h > -0.0410377667 && vsop87_elts.h < -0.0410377665);
    assert!(vsop87_elts.q > 0.0103755321 && vsop87_elts.q < 0.0103755323);
    assert!(vsop87_elts.p > 0.01281888 && vsop87_elts.p < 0.01281963);

    let vsop87_elts = vsop87::mars(2232395.0);

    assert!(vsop87_elts.a > 1.5237000650 && vsop87_elts.a < 1.5237000652);
    assert!(vsop87_elts.l > 6.1719713982 && vsop87_elts.l < 6.1719713984);
    assert!(vsop87_elts.k > 0.0831244747 && vsop87_elts.k < 0.0831244749);
    assert!(vsop87_elts.h > -0.0415495115 && vsop87_elts.h < -0.0415495113);
    assert!(vsop87_elts.q > 0.0103538428 && vsop87_elts.q < 0.0103538430);
    assert!(vsop87_elts.p > 0.01292515 && vsop87_elts.p < 0.01292591);

    let vsop87_elts = vsop87::mars(2195870.0);

    assert!(vsop87_elts.a > 1.5236148014 && vsop87_elts.a < 1.5236148016);
    assert!(vsop87_elts.l > 5.1194225687 && vsop87_elts.l < 5.1194225689);
    assert!(vsop87_elts.k > 0.0825058310 && vsop87_elts.k < 0.0825058312);
    assert!(vsop87_elts.h > -0.0421055437 && vsop87_elts.h < -0.0421055435);
    assert!(vsop87_elts.q > 0.0103313654 && vsop87_elts.q < 0.0103313656);
    assert!(vsop87_elts.p > 0.01303122 && vsop87_elts.p < 0.01303198);

    let vsop87_elts = vsop87::mars(2159345.0);

    assert!(vsop87_elts.a > 1.5237578877 && vsop87_elts.a < 1.5237578879);
    assert!(vsop87_elts.l > 4.0669853278 && vsop87_elts.l < 4.0669853280);
    assert!(vsop87_elts.k > 0.0821906316 && vsop87_elts.k < 0.0821906318);
    assert!(vsop87_elts.h > -0.0427917583 && vsop87_elts.h < -0.0427917581);
    assert!(vsop87_elts.q > 0.0103081045 && vsop87_elts.q < 0.0103081047);
    assert!(vsop87_elts.p > 0.01313608 && vsop87_elts.p < 0.01313684);

    let vsop87_elts = vsop87::mars(2122820.0);

    assert!(vsop87_elts.a > 1.5236045524 && vsop87_elts.a < 1.5236045526);
    assert!(vsop87_elts.l > 3.0145860937 && vsop87_elts.l < 3.0145860939);
    assert!(vsop87_elts.k > 0.0818247407 && vsop87_elts.k < 0.0818247409);
    assert!(vsop87_elts.h > -0.0434649475 && vsop87_elts.h < -0.0434649473);
    assert!(vsop87_elts.q > 0.0102833000 && vsop87_elts.q < 0.0102833002);
    assert!(vsop87_elts.p > 0.01324009 && vsop87_elts.p < 0.01324085);
}

#[test]
fn it_jupiter() {
    let vsop87_elts = vsop87::jupiter(2451545.0);

    assert!(vsop87_elts.a > 5.2042662907 && vsop87_elts.a < 5.2042662909);
    assert!(vsop87_elts.l > 0.5999772954 && vsop87_elts.l < 0.5999772956);
    assert!(vsop87_elts.k > 0.0469877115 && vsop87_elts.k < 0.0469877117);
    assert!(vsop87_elts.h > 0.0130817657 && vsop87_elts.h < 0.0130817659);
    assert!(vsop87_elts.q > -0.0020729463 && vsop87_elts.q < -0.0020729461);
    assert!(vsop87_elts.p > 0.01119395 && vsop87_elts.p < 0.01119471);

    let vsop87_elts = vsop87::jupiter(2415020.0);

    assert!(vsop87_elts.a > 5.2028202640 && vsop87_elts.a < 5.2028202642);
    assert!(vsop87_elts.l > 4.1841549083 && vsop87_elts.l < 4.1841549085);
    assert!(vsop87_elts.k > 0.0473151972 && vsop87_elts.k < 0.0473151974);
    assert!(vsop87_elts.h > 0.0115865095 && vsop87_elts.h < 0.0115865097);
    assert!(vsop87_elts.q > -0.0020327889 && vsop87_elts.q < -0.0020327887);
    assert!(vsop87_elts.p > 0.01121514 && vsop87_elts.p < 0.01121590);

    let vsop87_elts = vsop87::jupiter(2378495.0);

    assert!(vsop87_elts.a > 5.2027276672 && vsop87_elts.a < 5.2027276674);
    assert!(vsop87_elts.l > 1.4820596291 && vsop87_elts.l < 1.4820596293);
    assert!(vsop87_elts.k > 0.0464780412 && vsop87_elts.k < 0.0464780414);
    assert!(vsop87_elts.h > 0.0116460263 && vsop87_elts.h < 0.0116460265);
    assert!(vsop87_elts.q > -0.0019921307 && vsop87_elts.q < -0.0019921305);
    assert!(vsop87_elts.p > 0.01123447 && vsop87_elts.p < 0.01123523);

    let vsop87_elts = vsop87::jupiter(2341970.0);

    assert!(vsop87_elts.a > 5.2019341868 && vsop87_elts.a < 5.2019341870);
    assert!(vsop87_elts.l > 5.0599431121 && vsop87_elts.l < 5.0599431123);
    assert!(vsop87_elts.k > 0.0475018339 && vsop87_elts.k < 0.0475018341);
    assert!(vsop87_elts.h > 0.0119836367 && vsop87_elts.h < 0.0119836369);
    assert!(vsop87_elts.q > -0.0019621322 && vsop87_elts.q < -0.0019621320);
    assert!(vsop87_elts.p > 0.01125575 && vsop87_elts.p < 0.01125651);

    let vsop87_elts = vsop87::jupiter(2305445.0);

    assert!(vsop87_elts.a > 5.2018720769 && vsop87_elts.a < 5.2018720771);
    assert!(vsop87_elts.l > 2.3540228341 && vsop87_elts.l < 2.3540228343);
    assert!(vsop87_elts.k > 0.0468736015 && vsop87_elts.k < 0.0468736017);
    assert!(vsop87_elts.h > 0.0103577795 && vsop87_elts.h < 0.0103577797);
    assert!(vsop87_elts.q > -0.0019360509 && vsop87_elts.q < -0.0019360507);
    assert!(vsop87_elts.p > 0.01127707 && vsop87_elts.p < 0.01127783);

    let vsop87_elts = vsop87::jupiter(2268920.0);

    assert!(vsop87_elts.a > 5.2018353755 && vsop87_elts.a < 5.2018353757);
    assert!(vsop87_elts.l > 5.9291206443 && vsop87_elts.l < 5.9291206445);
    assert!(vsop87_elts.k > 0.0457181531 && vsop87_elts.k < 0.0457181533);
    assert!(vsop87_elts.h > 0.0107163503 && vsop87_elts.h < 0.0107163505);
    assert!(vsop87_elts.q > -0.0019130012 && vsop87_elts.q < -0.0019130010);
    assert!(vsop87_elts.p > 0.01129724 && vsop87_elts.p < 0.01129800);

    let vsop87_elts = vsop87::jupiter(2232395.0);

    assert!(vsop87_elts.a > 5.2016905938 && vsop87_elts.a < 5.2016905940);
    assert!(vsop87_elts.l > 3.2222446781 && vsop87_elts.l < 3.2222446783);
    assert!(vsop87_elts.k > 0.0468278448 && vsop87_elts.k < 0.0468278450);
    assert!(vsop87_elts.h > 0.0106167943 && vsop87_elts.h < 0.0106167945);
    assert!(vsop87_elts.q > -0.0018866373 && vsop87_elts.q < -0.0018866371);
    assert!(vsop87_elts.p > 0.01132280 && vsop87_elts.p < 0.01132356);

    let vsop87_elts = vsop87::jupiter(2195870.0);

    assert!(vsop87_elts.a > 5.2022541042 && vsop87_elts.a < 5.2022541044);
    assert!(vsop87_elts.l > 0.5183166214 && vsop87_elts.l < 0.5183166216);
    assert!(vsop87_elts.k > 0.0456561945 && vsop87_elts.k < 0.0456561947);
    assert!(vsop87_elts.h > 0.0099030073 && vsop87_elts.h < 0.0099030075);
    assert!(vsop87_elts.q > -0.0018595889 && vsop87_elts.q < -0.0018595887);
    assert!(vsop87_elts.p > 0.01135867 && vsop87_elts.p < 0.01135943);

    let vsop87_elts = vsop87::jupiter(2159345.0);

    assert!(vsop87_elts.a > 5.2025950507 && vsop87_elts.a < 5.2025950509);
    assert!(vsop87_elts.l > 4.0986003153 && vsop87_elts.l < 4.0986003155);
    assert!(vsop87_elts.k > 0.0451688311 && vsop87_elts.k < 0.0451688313);
    assert!(vsop87_elts.h > 0.0110668122 && vsop87_elts.h < 0.0110668124);
    assert!(vsop87_elts.q > -0.0018350865 && vsop87_elts.q < -0.0018350863);
    assert!(vsop87_elts.p > 0.01139106 && vsop87_elts.p < 0.01139182);

    let vsop87_elts = vsop87::jupiter(2122820.0);

    assert!(vsop87_elts.a > 5.2026637692 && vsop87_elts.a < 5.2026637694);
    assert!(vsop87_elts.l > 1.3989872238 && vsop87_elts.l < 1.3989872240);
    assert!(vsop87_elts.k > 0.0462126898 && vsop87_elts.k < 0.0462126900);
    assert!(vsop87_elts.h > 0.0102292159 && vsop87_elts.h < 0.0102292161);
    assert!(vsop87_elts.q > -0.0018026305 && vsop87_elts.q < -0.0018026303);
    assert!(vsop87_elts.p > 0.01141772 && vsop87_elts.p < 0.01141848);
}

#[test]
fn it_saturn() {
    let vsop87_elts = vsop87::saturn(2451545.0);

    assert!(vsop87_elts.a > 9.5820161866 && vsop87_elts.a < 9.5820161868);
    assert!(vsop87_elts.l > 0.8727430949 && vsop87_elts.l < 0.8727430951);
    assert!(vsop87_elts.k > 0.0003336008 && vsop87_elts.k < 0.0003336010);
    assert!(vsop87_elts.h > 0.0557224685 && vsop87_elts.h < 0.0557224687);
    assert!(vsop87_elts.q > -0.0086968780 && vsop87_elts.q < -0.0086968778);
    assert!(vsop87_elts.p > 0.01986563 && vsop87_elts.p < 0.01986639);

    let vsop87_elts = vsop87::saturn(2415020.0);

    assert!(vsop87_elts.a > 9.5797975825 && vsop87_elts.a < 9.5797975827);
    assert!(vsop87_elts.l > 4.6635485633 && vsop87_elts.l < 4.6635485635);
    assert!(vsop87_elts.k > -0.0037561571 && vsop87_elts.k < -0.0037561569);
    assert!(vsop87_elts.h > 0.0510499910 && vsop87_elts.h < 0.0510499912);
    assert!(vsop87_elts.q > -0.0087942465 && vsop87_elts.q < -0.0087942463);
    assert!(vsop87_elts.p > 0.01980654 && vsop87_elts.p < 0.01980730);

    let vsop87_elts = vsop87::saturn(2378495.0);

    assert!(vsop87_elts.a > 9.5845294674 && vsop87_elts.a < 9.5845294676);
    assert!(vsop87_elts.l > 2.1792108199 && vsop87_elts.l < 2.1792108201);
    assert!(vsop87_elts.k > -0.0040674040 && vsop87_elts.k < -0.0040674038);
    assert!(vsop87_elts.h > 0.0594660901 && vsop87_elts.h < 0.0594660903);
    assert!(vsop87_elts.q > -0.0088957952 && vsop87_elts.q < -0.0088957950);
    assert!(vsop87_elts.p > 0.01975409 && vsop87_elts.p < 0.01975485);

    let vsop87_elts = vsop87::saturn(2341970.0);

    assert!(vsop87_elts.a > 9.5793512112 && vsop87_elts.a < 9.5793512114);
    assert!(vsop87_elts.l > 5.9857401998 && vsop87_elts.l < 5.9857402000);
    assert!(vsop87_elts.k > -0.0007610587 && vsop87_elts.k < -0.0007610585);
    assert!(vsop87_elts.h > 0.0541118731 && vsop87_elts.h < 0.0541118733);
    assert!(vsop87_elts.q > -0.0089677722 && vsop87_elts.q < -0.0089677720);
    assert!(vsop87_elts.p > 0.01970184 && vsop87_elts.p < 0.01970260);

    let vsop87_elts = vsop87::saturn(2305445.0);

    assert!(vsop87_elts.a > 9.5727100002 && vsop87_elts.a < 9.5727100004);
    assert!(vsop87_elts.l > 3.5107821038 && vsop87_elts.l < 3.5107821040);
    assert!(vsop87_elts.k > -0.0048218813 && vsop87_elts.k < -0.0048218811);
    assert!(vsop87_elts.h > 0.0575514202 && vsop87_elts.h < 0.0575514204);
    assert!(vsop87_elts.q > -0.0090348990 && vsop87_elts.q < -0.0090348988);
    assert!(vsop87_elts.p > 0.01965756 && vsop87_elts.p < 0.01965832);

    let vsop87_elts = vsop87::saturn(2268920.0);

    assert!(vsop87_elts.a > 9.5665592834 && vsop87_elts.a < 9.5665592836);
    assert!(vsop87_elts.l > 1.0414908681 && vsop87_elts.l < 1.0414908683);
    assert!(vsop87_elts.k > 0.0023388087 && vsop87_elts.k < 0.0023388089);
    assert!(vsop87_elts.h > 0.0601498959 && vsop87_elts.h < 0.0601498961);
    assert!(vsop87_elts.q > -0.0090989758 && vsop87_elts.q < -0.0090989756);
    assert!(vsop87_elts.p > 0.01961148 && vsop87_elts.p < 0.01961224);

    let vsop87_elts = vsop87::saturn(2232395.0);

    assert!(vsop87_elts.a > 9.5588545246 && vsop87_elts.a < 9.5588545248);
    assert!(vsop87_elts.l > 4.8521539279 && vsop87_elts.l < 4.8521539281);
    assert!(vsop87_elts.k > -0.0000063021 && vsop87_elts.k < -0.0000063019);
    assert!(vsop87_elts.h > 0.0578024218 && vsop87_elts.h < 0.0578024220);
    assert!(vsop87_elts.q > -0.0091690231 && vsop87_elts.q < -0.0091690229);
    assert!(vsop87_elts.p > 0.01954496 && vsop87_elts.p < 0.01954572);

    let vsop87_elts = vsop87::saturn(2195870.0);

    assert!(vsop87_elts.a > 9.5448204346 && vsop87_elts.a < 9.5448204348);
    assert!(vsop87_elts.l > 2.3727005008 && vsop87_elts.l < 2.3727005010);
    assert!(vsop87_elts.k > 0.0022420512 && vsop87_elts.k < 0.0022420514);
    assert!(vsop87_elts.h > 0.0594072383 && vsop87_elts.h < 0.0594072385);
    assert!(vsop87_elts.q > -0.0092439695 && vsop87_elts.q < -0.0092439693);
    assert!(vsop87_elts.p > 0.01945077 && vsop87_elts.p < 0.01945153);

    let vsop87_elts = vsop87::saturn(2159345.0);

    assert!(vsop87_elts.a > 9.5363776410 && vsop87_elts.a < 9.5363776412);
    assert!(vsop87_elts.l > 6.1737761791 && vsop87_elts.l < 6.1737761793);
    assert!(vsop87_elts.k > 0.0034673265 && vsop87_elts.k < 0.0034673267);
    assert!(vsop87_elts.h > 0.0565365311 && vsop87_elts.h < 0.0565365313);
    assert!(vsop87_elts.q > -0.0093073645 && vsop87_elts.q < -0.0093073643);
    assert!(vsop87_elts.p > 0.01936955 && vsop87_elts.p < 0.01937031);

    let vsop87_elts = vsop87::saturn(2122820.0);

    assert!(vsop87_elts.a > 9.5316426699 && vsop87_elts.a < 9.5316426701);
    assert!(vsop87_elts.l > 3.6837162324 && vsop87_elts.l < 3.6837162326);
    assert!(vsop87_elts.k > 0.0034527066 && vsop87_elts.k < 0.0034527068);
    assert!(vsop87_elts.h > 0.0581970244 && vsop87_elts.h < 0.0581970246);
    assert!(vsop87_elts.q > -0.0093872673 && vsop87_elts.q < -0.0093872671);
    assert!(vsop87_elts.p > 0.01930558 && vsop87_elts.p < 0.01930634);
}

#[test]
fn it_uranus() {
    let vsop87_elts = vsop87::uranus(2451545.0);

    assert!(vsop87_elts.a > 19.2294229490 && vsop87_elts.a < 19.2294229492);
    assert!(vsop87_elts.l > 5.4713756705 && vsop87_elts.l < 5.4713756707);
    assert!(vsop87_elts.k > -0.0438022684 && vsop87_elts.k < -0.0438022682);
    assert!(vsop87_elts.h > 0.0073046994 && vsop87_elts.h < 0.0073046996);
    assert!(vsop87_elts.q > 0.0018594790 && vsop87_elts.q < 0.0018594792);
    assert!(vsop87_elts.p > 0.00648001 && vsop87_elts.p < 0.00648077);

    let vsop87_elts = vsop87::uranus(2415020.0);

    assert!(vsop87_elts.a > 19.3136226839 && vsop87_elts.a < 19.3136226841);
    assert!(vsop87_elts.l > 4.2716633732 && vsop87_elts.l < 4.2716633734);
    assert!(vsop87_elts.k > -0.0488881225 && vsop87_elts.k < -0.0488881223);
    assert!(vsop87_elts.h > 0.0023646937 && vsop87_elts.h < 0.0023646939);
    assert!(vsop87_elts.q > 0.0018638195 && vsop87_elts.q < 0.0018638197);
    assert!(vsop87_elts.p > 0.00650629 && vsop87_elts.p < 0.00650705);

    let vsop87_elts = vsop87::uranus(2378495.0);

    assert!(vsop87_elts.a > 19.2386333587 && vsop87_elts.a < 19.2386333589);
    assert!(vsop87_elts.l > 3.0730118894 && vsop87_elts.l < 3.0730118896);
    assert!(vsop87_elts.k > -0.0483021691 && vsop87_elts.k < -0.0483021689);
    assert!(vsop87_elts.h > 0.0101661419 && vsop87_elts.h < 0.0101661421);
    assert!(vsop87_elts.q > 0.0018715045 && vsop87_elts.q < 0.0018715047);
    assert!(vsop87_elts.p > 0.00652151 && vsop87_elts.p < 0.00652227);

    let vsop87_elts = vsop87::uranus(2341970.0);

    assert!(vsop87_elts.a > 19.1230558110 && vsop87_elts.a < 19.1230558112);
    assert!(vsop87_elts.l > 1.8812287778 && vsop87_elts.l < 1.8812287780);
    assert!(vsop87_elts.k > -0.0447817248 && vsop87_elts.k < -0.0447817246);
    assert!(vsop87_elts.h > 0.0053751773 && vsop87_elts.h < 0.0053751775);
    assert!(vsop87_elts.q > 0.0019049676 && vsop87_elts.q < 0.0019049678);
    assert!(vsop87_elts.p > 0.00651013 && vsop87_elts.p < 0.00651089);

    let vsop87_elts = vsop87::uranus(2305445.0);

    assert!(vsop87_elts.a > 19.1822072678 && vsop87_elts.a < 19.1822072680);
    assert!(vsop87_elts.l > 0.6902152336 && vsop87_elts.l < 0.6902152338);
    assert!(vsop87_elts.k > -0.0464228380 && vsop87_elts.k < -0.0464228378);
    assert!(vsop87_elts.h > 0.0091244199 && vsop87_elts.h < 0.0091244201);
    assert!(vsop87_elts.q > 0.0019242447 && vsop87_elts.q < 0.0019242449);
    assert!(vsop87_elts.p > 0.00654129 && vsop87_elts.p < 0.00654205);

    let vsop87_elts = vsop87::uranus(2268920.0);

    assert!(vsop87_elts.a > 19.2962467890 && vsop87_elts.a < 19.2962467892);
    assert!(vsop87_elts.l > 5.7758134684 && vsop87_elts.l < 5.7758134686);
    assert!(vsop87_elts.k > -0.0401255855 && vsop87_elts.k < -0.0401255853);
    assert!(vsop87_elts.h > 0.0056751507 && vsop87_elts.h < 0.0056751509);
    assert!(vsop87_elts.q > 0.0019109734 && vsop87_elts.q < 0.0019109736);
    assert!(vsop87_elts.p > 0.00654797 && vsop87_elts.p < 0.00654873);

    let vsop87_elts = vsop87::uranus(2232395.0);

    assert!(vsop87_elts.a > 19.2497356422 && vsop87_elts.a < 19.2497356424);
    assert!(vsop87_elts.l > 4.5777275752 && vsop87_elts.l < 4.5777275754);
    assert!(vsop87_elts.k > -0.0466529112 && vsop87_elts.k < -0.0466529110);
    assert!(vsop87_elts.h > 0.0051308956 && vsop87_elts.h < 0.0051308958);
    assert!(vsop87_elts.q > 0.0019206656 && vsop87_elts.q < 0.0019206658);
    assert!(vsop87_elts.p > 0.00655819 && vsop87_elts.p < 0.00655895);

    let vsop87_elts = vsop87::uranus(2195870.0);

    assert!(vsop87_elts.a > 19.1545703279 && vsop87_elts.a < 19.1545703281);
    assert!(vsop87_elts.l > 3.3858021155 && vsop87_elts.l < 3.3858021157);
    assert!(vsop87_elts.k > -0.0434958030 && vsop87_elts.k < -0.0434958028);
    assert!(vsop87_elts.h > 0.0088974145 && vsop87_elts.h < 0.0088974147);
    assert!(vsop87_elts.q > 0.0019372306 && vsop87_elts.q < 0.0019372308);
    assert!(vsop87_elts.p > 0.00658337 && vsop87_elts.p < 0.00658413);

    let vsop87_elts = vsop87::uranus(2159345.0);

    assert!(vsop87_elts.a > 19.1811347111 && vsop87_elts.a < 19.1811347113);
    assert!(vsop87_elts.l > 2.1967520045 && vsop87_elts.l < 2.1967520047);
    assert!(vsop87_elts.k > -0.0456007263 && vsop87_elts.k < -0.0456007261);
    assert!(vsop87_elts.h > 0.0068443676 && vsop87_elts.h < 0.0068443678);
    assert!(vsop87_elts.q > 0.0019640435 && vsop87_elts.q < 0.0019640437);
    assert!(vsop87_elts.p > 0.00658836 && vsop87_elts.p < 0.00658912);

    let vsop87_elts = vsop87::uranus(2122820.0);

    assert!(vsop87_elts.a > 19.2685452352 && vsop87_elts.a < 19.2685452354);
    assert!(vsop87_elts.l > 1.0026901572 && vsop87_elts.l < 1.0026901574);
    assert!(vsop87_elts.k > -0.0428389229 && vsop87_elts.k < -0.0428389227);
    assert!(vsop87_elts.h > 0.0113976085 && vsop87_elts.h < 0.0113976087);
    assert!(vsop87_elts.q > 0.0019824674 && vsop87_elts.q < 0.0019824676);
    assert!(vsop87_elts.p > 0.00660903 && vsop87_elts.p < 0.00660979);
}

#[test]
fn it_neptune() {
    let vsop87_elts = vsop87::neptune(2451545.0);

    assert!(vsop87_elts.a > 30.1036169705 && vsop87_elts.a < 30.1036169707);
    assert!(vsop87_elts.l > 5.3268987908 && vsop87_elts.l < 5.3268987910);
    assert!(vsop87_elts.k > 0.0089053320 && vsop87_elts.k < 0.0089053322);
    assert!(vsop87_elts.h > 0.0068181683 && vsop87_elts.h < 0.0068181685);
    assert!(vsop87_elts.q > -0.0102818995 && vsop87_elts.q < -0.0102818993);
    assert!(vsop87_elts.p > 0.01150175 && vsop87_elts.p < 0.01150251);

    let vsop87_elts = vsop87::neptune(2415020.0);

    assert!(vsop87_elts.a > 29.9473727789 && vsop87_elts.a < 29.9473727791);
    assert!(vsop87_elts.l > 1.5103401220 && vsop87_elts.l < 1.5103401222);
    assert!(vsop87_elts.k > 0.0048403714 && vsop87_elts.k < 0.0048403716);
    assert!(vsop87_elts.h > 0.0022828947 && vsop87_elts.h < 0.0022828949);
    assert!(vsop87_elts.q > -0.0102936855 && vsop87_elts.q < -0.0102936853);
    assert!(vsop87_elts.p > 0.01150022 && vsop87_elts.p < 0.01150098);

    let vsop87_elts = vsop87::neptune(2378495.0);

    assert!(vsop87_elts.a > 29.9925541926 && vsop87_elts.a < 29.9925541928);
    assert!(vsop87_elts.l > 3.9743817918 && vsop87_elts.l < 3.9743817920);
    assert!(vsop87_elts.k > 0.0060997431 && vsop87_elts.k < 0.0060997433);
    assert!(vsop87_elts.h > 0.0092474382 && vsop87_elts.h < 0.0092474384);
    assert!(vsop87_elts.q > -0.0102935575 && vsop87_elts.q < -0.0102935573);
    assert!(vsop87_elts.p > 0.01150802 && vsop87_elts.p < 0.01150878);

    let vsop87_elts = vsop87::neptune(2341970.0);

    assert!(vsop87_elts.a > 30.1627820094 && vsop87_elts.a < 30.1627820096);
    assert!(vsop87_elts.l > 0.1605390710 && vsop87_elts.l < 0.1605390712);
    assert!(vsop87_elts.k > 0.0091870580 && vsop87_elts.k < 0.0091870582);
    assert!(vsop87_elts.h > 0.0043333831 && vsop87_elts.h < 0.0043333833);
    assert!(vsop87_elts.q > -0.0102857259 && vsop87_elts.q < -0.0102857257);
    assert!(vsop87_elts.p > 0.01151010 && vsop87_elts.p < 0.01151086);

    let vsop87_elts = vsop87::neptune(2305445.0);

    assert!(vsop87_elts.a > 30.2702161622 && vsop87_elts.a < 30.2702161624);
    assert!(vsop87_elts.l > 2.6344819843 && vsop87_elts.l < 2.6344819845);
    assert!(vsop87_elts.k > 0.0001266623 && vsop87_elts.k < 0.0001266625);
    assert!(vsop87_elts.h > 0.0095018713 && vsop87_elts.h < 0.0095018715);
    assert!(vsop87_elts.q > -0.0102752821 && vsop87_elts.q < -0.0102752819);
    assert!(vsop87_elts.p > 0.01149703 && vsop87_elts.p < 0.01149779);

    let vsop87_elts = vsop87::neptune(2268920.0);

    assert!(vsop87_elts.a > 30.1963044187 && vsop87_elts.a < 30.1963044189);
    assert!(vsop87_elts.l > 5.1088676118 && vsop87_elts.l < 5.1088676120);
    assert!(vsop87_elts.k > 0.0091964091 && vsop87_elts.k < 0.0091964093);
    assert!(vsop87_elts.h > 0.0031103619 && vsop87_elts.h < 0.0031103621);
    assert!(vsop87_elts.q > -0.0102800265 && vsop87_elts.q < -0.0102800263);
    assert!(vsop87_elts.p > 0.01148076 && vsop87_elts.p < 0.01148152);

    let vsop87_elts = vsop87::neptune(2232395.0);

    assert!(vsop87_elts.a > 30.0205469235 && vsop87_elts.a < 30.0205469237);
    assert!(vsop87_elts.l > 1.2942368464 && vsop87_elts.l < 1.2942368466);
    assert!(vsop87_elts.k > 0.0036280450 && vsop87_elts.k < 0.0036280452);
    assert!(vsop87_elts.h > 0.0054820844 && vsop87_elts.h < 0.0054820846);
    assert!(vsop87_elts.q > -0.0102966028 && vsop87_elts.q < -0.0102966026);
    assert!(vsop87_elts.p > 0.01147638 && vsop87_elts.p < 0.01147714);

    let vsop87_elts = vsop87::neptune(2195870.0);

    assert!(vsop87_elts.a > 29.9660361001 && vsop87_elts.a < 29.9660361003);
    assert!(vsop87_elts.l > 3.7591524119 && vsop87_elts.l < 3.7591524121);
    assert!(vsop87_elts.k > 0.0080747908 && vsop87_elts.k < 0.0080747910);
    assert!(vsop87_elts.h > 0.0084977048 && vsop87_elts.h < 0.0084977050);
    assert!(vsop87_elts.q > -0.0103068591 && vsop87_elts.q < -0.0103068589);
    assert!(vsop87_elts.p > 0.01148319 && vsop87_elts.p < 0.01148395);

    let vsop87_elts = vsop87::neptune(2159345.0);

    assert!(vsop87_elts.a > 30.0586108829 && vsop87_elts.a < 30.0586108831);
    assert!(vsop87_elts.l > 6.2253193739 && vsop87_elts.l < 6.2253193741);
    assert!(vsop87_elts.k > 0.0053615686 && vsop87_elts.k < 0.0053615688);
    assert!(vsop87_elts.h > 0.0046522989 && vsop87_elts.h < 0.0046522991);
    assert!(vsop87_elts.q > -0.0103020951 && vsop87_elts.q < -0.0103020949);
    assert!(vsop87_elts.p > 0.01149464 && vsop87_elts.p < 0.01149540);

    let vsop87_elts = vsop87::neptune(2122820.0);

    assert!(vsop87_elts.a > 30.2002490333 && vsop87_elts.a < 30.2002490335);
    assert!(vsop87_elts.l > 2.4115747748 && vsop87_elts.l < 2.4115747750);
    assert!(vsop87_elts.k > 0.0032261109 && vsop87_elts.k < 0.0032261111);
    assert!(vsop87_elts.h > 0.0104510183 && vsop87_elts.h < 0.0104510185);
    assert!(vsop87_elts.q > -0.0102898481 && vsop87_elts.q < -0.0102898479);
    assert!(vsop87_elts.p > 0.01149039 && vsop87_elts.p < 0.01149115);
}

#[test]
fn it_earth_moon() {
    let vsop87_elts = vsop87::earth_moon(2451545.0);

    assert!(vsop87_elts.a > 0.9999964221 && vsop87_elts.a < 0.9999964223);
    assert!(vsop87_elts.l > 1.7534128815 && vsop87_elts.l < 1.7534128817);
    assert!(vsop87_elts.k > -0.0037339067 && vsop87_elts.k < -0.0037339065);
    assert!(vsop87_elts.h > 0.0162796345 && vsop87_elts.h < 0.0162796347);
    assert!(vsop87_elts.q > -0.0000006037 && vsop87_elts.q < -0.0000006035);
    assert!(vsop87_elts.p > 0.00000025 && vsop87_elts.p < 0.00000101);

    let vsop87_elts = vsop87::earth_moon(2415020.0);

    assert!(vsop87_elts.a > 0.9999996844 && vsop87_elts.a < 0.9999996846);
    assert!(vsop87_elts.l > 1.7643937667 && vsop87_elts.l < 1.7643937669);
    assert!(vsop87_elts.k > -0.0036507092 && vsop87_elts.k < -0.0036507090);
    assert!(vsop87_elts.h > 0.0163633889 && vsop87_elts.h < 0.0163633891);
    assert!(vsop87_elts.q > 0.0001135348 && vsop87_elts.q < 0.0001135350);
    assert!(vsop87_elts.p > -0.00001064 && vsop87_elts.p < -0.00000988);

    let vsop87_elts = vsop87::earth_moon(2378495.0);

    assert!(vsop87_elts.a > 1.0000244437 && vsop87_elts.a < 1.0000244439);
    assert!(vsop87_elts.l > 1.7753590238 && vsop87_elts.l < 1.7753590240);
    assert!(vsop87_elts.k > -0.0035970436 && vsop87_elts.k < -0.0035970434);
    assert!(vsop87_elts.h > 0.0164283611 && vsop87_elts.h < 0.0164283613);
    assert!(vsop87_elts.q > 0.0002263287 && vsop87_elts.q < 0.0002263289);
    assert!(vsop87_elts.p > -0.00001932 && vsop87_elts.p < -0.00001856);

    let vsop87_elts = vsop87::earth_moon(2341970.0);

    assert!(vsop87_elts.a > 0.9999991295 && vsop87_elts.a < 0.9999991297);
    assert!(vsop87_elts.l > 1.7863136093 && vsop87_elts.l < 1.7863136095);
    assert!(vsop87_elts.k > -0.0035112303 && vsop87_elts.k < -0.0035112301);
    assert!(vsop87_elts.h > 0.0164802094 && vsop87_elts.h < 0.0164802096);
    assert!(vsop87_elts.q > 0.0003422697 && vsop87_elts.q < 0.0003422699);
    assert!(vsop87_elts.p > -0.00002679 && vsop87_elts.p < -0.00002603);

    let vsop87_elts = vsop87::earth_moon(2305445.0);

    assert!(vsop87_elts.a > 0.9999995277 && vsop87_elts.a < 0.9999995279);
    assert!(vsop87_elts.l > 1.7972736109 && vsop87_elts.l < 1.7972736111);
    assert!(vsop87_elts.k > -0.0034204916 && vsop87_elts.k < -0.0034204914);
    assert!(vsop87_elts.h > 0.0165004096 && vsop87_elts.h < 0.0165004098);
    assert!(vsop87_elts.q > 0.0004561548 && vsop87_elts.q < 0.0004561550);
    assert!(vsop87_elts.p > -0.00003406 && vsop87_elts.p < -0.00003330);

    let vsop87_elts = vsop87::earth_moon(2268920.0);

    assert!(vsop87_elts.a > 0.9999866241 && vsop87_elts.a < 0.9999866243);
    assert!(vsop87_elts.l > 1.8082350760 && vsop87_elts.l < 1.8082350762);
    assert!(vsop87_elts.k > -0.0033449833 && vsop87_elts.k < -0.0033449831);
    assert!(vsop87_elts.h > 0.0165522472 && vsop87_elts.h < 0.0165522474);
    assert!(vsop87_elts.q > 0.0005701580 && vsop87_elts.q < 0.0005701582);
    assert!(vsop87_elts.p > -0.00003926 && vsop87_elts.p < -0.00003850);

    let vsop87_elts = vsop87::earth_moon(2232395.0);

    assert!(vsop87_elts.a > 0.9999994546 && vsop87_elts.a < 0.9999994548);
    assert!(vsop87_elts.l > 1.8191248026 && vsop87_elts.l < 1.8191248028);
    assert!(vsop87_elts.k > -0.0032035188 && vsop87_elts.k < -0.0032035186);
    assert!(vsop87_elts.h > 0.0166424875 && vsop87_elts.h < 0.0166424877);
    assert!(vsop87_elts.q > 0.0006860762 && vsop87_elts.q < 0.0006860764);
    assert!(vsop87_elts.p > -0.00004422 && vsop87_elts.p < -0.00004346);

    let vsop87_elts = vsop87::earth_moon(2195870.0);

    assert!(vsop87_elts.a > 0.9999859332 && vsop87_elts.a < 0.9999859334);
    assert!(vsop87_elts.l > 1.8300786646 && vsop87_elts.l < 1.8300786648);
    assert!(vsop87_elts.k > -0.0031249208 && vsop87_elts.k < -0.0031249206);
    assert!(vsop87_elts.h > 0.0166697591 && vsop87_elts.h < 0.0166697593);
    assert!(vsop87_elts.q > 0.0007994894 && vsop87_elts.q < 0.0007994896);
    assert!(vsop87_elts.p > -0.00004884 && vsop87_elts.p < -0.00004808);

    let vsop87_elts = vsop87::earth_moon(2159345.0);

    assert!(vsop87_elts.a > 1.0000057989 && vsop87_elts.a < 1.0000057991);
    assert!(vsop87_elts.l > 1.8410570603 && vsop87_elts.l < 1.8410570605);
    assert!(vsop87_elts.k > -0.0030431505 && vsop87_elts.k < -0.0030431503);
    assert!(vsop87_elts.h > 0.0167901446 && vsop87_elts.h < 0.0167901448);
    assert!(vsop87_elts.q > 0.0009147432 && vsop87_elts.q < 0.0009147434);
    assert!(vsop87_elts.p > -0.00005091 && vsop87_elts.p < -0.00005015);

    let vsop87_elts = vsop87::earth_moon(2122820.0);

    assert!(vsop87_elts.a > 1.0000134925 && vsop87_elts.a < 1.0000134927);
    assert!(vsop87_elts.l > 1.8519621672 && vsop87_elts.l < 1.8519621674);
    assert!(vsop87_elts.k > -0.0029638176 && vsop87_elts.k < -0.0029638174);
    assert!(vsop87_elts.h > 0.0168402193 && vsop87_elts.h < 0.0168402195);
    assert!(vsop87_elts.q > 0.0010301900 && vsop87_elts.q < 0.0010301902);
    assert!(vsop87_elts.p > -0.00005346 && vsop87_elts.p < -0.00005270);
}
