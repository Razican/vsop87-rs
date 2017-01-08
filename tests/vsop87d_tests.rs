extern crate vsop87;
use vsop87::*;

#[test]
fn it_mercury() {
    let coordinates = vsop87d::mercury(2451545.0);

    assert!(coordinates.longitude() > 4.4293481035 && coordinates.longitude() < 4.4293481037);
    assert!(coordinates.latitude() > -0.0527573410 && coordinates.latitude() < -0.0527573408);
    assert!(coordinates.distance() > 0.46647110 && coordinates.distance() < 0.46647186);

    let coordinates = vsop87d::mercury(2415020.0);

    assert!(coordinates.longitude() > 3.4851161910 && coordinates.longitude() < 3.4851161912);
    assert!(coordinates.latitude() > 0.0565906172 && coordinates.latitude() < 0.0565906174);
    assert!(coordinates.distance() > 0.41834225 && coordinates.distance() < 0.41834301);

    let coordinates = vsop87d::mercury(2378495.0);

    assert!(coordinates.longitude() > 2.0737894887 && coordinates.longitude() < 2.0737894889);
    assert!(coordinates.latitude() > 0.1168184803 && coordinates.latitude() < 0.1168184805);
    assert!(coordinates.distance() > 0.32339057 && coordinates.distance() < 0.32339133);

    let coordinates = vsop87d::mercury(2341970.0);

    assert!(coordinates.longitude() > 0.1910149586 && coordinates.longitude() < 0.1910149588);
    assert!(coordinates.latitude() > -0.0682441257 && coordinates.latitude() < -0.0682441255);
    assert!(coordinates.distance() > 0.33815593 && coordinates.distance() < 0.33815669);

    let coordinates = vsop87d::mercury(2305445.0);

    assert!(coordinates.longitude() > 5.1836421819 && coordinates.longitude() < 5.1836421821);
    assert!(coordinates.latitude() > -0.1170914849 && coordinates.latitude() < -0.1170914847);
    assert!(coordinates.distance() > 0.43265140 && coordinates.distance() < 0.43265216);

    let coordinates = vsop87d::mercury(2268920.0);

    assert!(coordinates.longitude() > 4.2636517902 && coordinates.longitude() < 4.2636517904);
    assert!(coordinates.latitude() > -0.0457048517 && coordinates.latitude() < -0.0457048515);
    assert!(coordinates.distance() > 0.46615201 && coordinates.distance() < 0.46615277);

    let coordinates = vsop87d::mercury(2232395.0);

    assert!(coordinates.longitude() > 3.3115600861 && coordinates.longitude() < 3.3115600863);
    assert!(coordinates.latitude() > 0.0639722346 && coordinates.latitude() < 0.0639722348);
    assert!(coordinates.distance() > 0.41523814 && coordinates.distance() < 0.41523890);

    let coordinates = vsop87d::mercury(2195870.0);

    assert!(coordinates.longitude() > 1.8738888758 && coordinates.longitude() < 1.8738888760);
    assert!(coordinates.latitude() > 0.1126774696 && coordinates.latitude() < 0.1126774698);
    assert!(coordinates.distance() > 0.32093624 && coordinates.distance() < 0.32093700);

    let coordinates = vsop87d::mercury(2159345.0);

    assert!(coordinates.longitude() > 6.2819826059 && coordinates.longitude() < 6.2819826061);
    assert!(coordinates.latitude() > -0.0768697085 && coordinates.latitude() < -0.0768697083);
    assert!(coordinates.distance() > 0.34143504 && coordinates.distance() < 0.34143581);

    let coordinates = vsop87d::mercury(2122820.0);

    assert!(coordinates.longitude() > 5.0128397763 && coordinates.longitude() < 5.0128397765);
    assert!(coordinates.latitude() > -0.1143275809 && coordinates.latitude() < -0.1143275807);
    assert!(coordinates.distance() > 0.43520594 && coordinates.distance() < 0.43520670);
}

#[test]
fn it_venus() {
    let coordinates = vsop87d::venus(2451545.0);

    assert!(coordinates.longitude() > 3.1870221832 && coordinates.longitude() < 3.1870221834);
    assert!(coordinates.latitude() > 0.0569782848 && coordinates.latitude() < 0.0569782850);
    assert!(coordinates.distance() > 0.72021255 && coordinates.distance() < 0.72021331);

    let coordinates = vsop87d::venus(2415020.0);

    assert!(coordinates.longitude() > 5.9749622237 && coordinates.longitude() < 5.9749622239);
    assert!(coordinates.latitude() > -0.0591260015 && coordinates.latitude() < -0.0591260013);
    assert!(coordinates.distance() > 0.72747156 && coordinates.distance() < 0.72747232);

    let coordinates = vsop87d::venus(2378495.0);

    assert!(coordinates.longitude() > 2.5083656667 && coordinates.longitude() < 2.5083656669);
    assert!(coordinates.latitude() > 0.0552309406 && coordinates.latitude() < 0.0552309408);
    assert!(coordinates.distance() > 0.71854695 && coordinates.distance() < 0.71854771);

    let coordinates = vsop87d::venus(2341970.0);

    assert!(coordinates.longitude() > 5.3115708035 && coordinates.longitude() < 5.3115708037);
    assert!(coordinates.latitude() > -0.0455979905 && coordinates.latitude() < -0.0455979903);
    assert!(coordinates.distance() > 0.72834037 && coordinates.distance() < 0.72834113);

    let coordinates = vsop87d::venus(2305445.0);

    assert!(coordinates.longitude() > 1.8291359616 && coordinates.longitude() < 1.8291359618);
    assert!(coordinates.latitude() > 0.0311394083 && coordinates.latitude() < 0.0311394085);
    assert!(coordinates.distance() > 0.71863712 && coordinates.distance() < 0.71863788);

    let coordinates = vsop87d::venus(2268920.0);

    assert!(coordinates.longitude() > 4.6495448743 && coordinates.longitude() < 4.6495448745);
    assert!(coordinates.latitude() > -0.0145437543 && coordinates.latitude() < -0.0145437541);
    assert!(coordinates.distance() > 0.72733600 && coordinates.distance() < 0.72733676);

    let coordinates = vsop87d::venus(2232395.0);

    assert!(coordinates.longitude() > 1.1527504142 && coordinates.longitude() < 1.1527504144);
    assert!(coordinates.latitude() > -0.0054100667 && coordinates.latitude() < -0.0054100665);
    assert!(coordinates.distance() > 0.72054247 && coordinates.distance() < 0.72054323);

    let coordinates = vsop87d::venus(2195870.0);

    assert!(coordinates.longitude() > 3.9850309908 && coordinates.longitude() < 3.9850309910);
    assert!(coordinates.latitude() > 0.0222342484 && coordinates.latitude() < 0.0222342486);
    assert!(coordinates.distance() > 0.72474374 && coordinates.distance() < 0.72474450);

    let coordinates = vsop87d::venus(2159345.0);

    assert!(coordinates.longitude() > 0.4804699930 && coordinates.longitude() < 0.4804699932);
    assert!(coordinates.latitude() > -0.0395505251 && coordinates.latitude() < -0.0395505249);
    assert!(coordinates.distance() > 0.72354267 && coordinates.distance() < 0.72354343);

    let coordinates = vsop87d::venus(2122820.0);

    assert!(coordinates.longitude() > 3.3145399294 && coordinates.longitude() < 3.3145399296);
    assert!(coordinates.latitude() > 0.0505016052 && coordinates.latitude() < 0.0505016054);
    assert!(coordinates.distance() > 0.72158160 && coordinates.distance() < 0.72158236);
}

#[test]
fn it_earth() {
    let coordinates = vsop87d::earth(2451545.0);

    assert!(coordinates.longitude() > 1.7519238680 && coordinates.longitude() < 1.7519238682);
    assert!(coordinates.latitude() > -0.0000039657 && coordinates.latitude() < -0.0000039655);
    assert!(coordinates.distance() > 0.98332730 && coordinates.distance() < 0.98332806);

    let coordinates = vsop87d::earth(2415020.0);

    assert!(coordinates.longitude() > 1.7391225562 && coordinates.longitude() < 1.7391225564);
    assert!(coordinates.latitude() > -0.0000005680 && coordinates.latitude() < -0.0000005678);
    assert!(coordinates.distance() > 0.98326860 && coordinates.distance() < 0.98326936);

    let coordinates = vsop87d::earth(2378495.0);

    assert!(coordinates.longitude() > 1.7262638915 && coordinates.longitude() < 1.7262638917);
    assert!(coordinates.latitude() > 0.0000002082 && coordinates.latitude() < 0.0000002084);
    assert!(coordinates.distance() > 0.98322705 && coordinates.distance() < 0.98322781);

    let coordinates = vsop87d::earth(2341970.0);

    assert!(coordinates.longitude() > 1.7134419104 && coordinates.longitude() < 1.7134419106);
    assert!(coordinates.latitude() > 0.0000025050 && coordinates.latitude() < 0.0000025052);
    assert!(coordinates.distance() > 0.98314946 && coordinates.distance() < 0.98315022);

    let coordinates = vsop87d::earth(2305445.0);

    assert!(coordinates.longitude() > 1.7006065937 && coordinates.longitude() < 1.7006065939);
    assert!(coordinates.latitude() > -0.0000016360 && coordinates.latitude() < -0.0000016358);
    assert!(coordinates.distance() > 0.98312506 && coordinates.distance() < 0.98312582);

    let coordinates = vsop87d::earth(2268920.0);

    assert!(coordinates.longitude() > 1.6877624959 && coordinates.longitude() < 1.6877624961);
    assert!(coordinates.latitude() > -0.0000020341 && coordinates.latitude() < -0.0000020339);
    assert!(coordinates.distance() > 0.98308130 && coordinates.distance() < 0.98308206);

    let coordinates = vsop87d::earth(2232395.0);

    assert!(coordinates.longitude() > 1.6750110960 && coordinates.longitude() < 1.6750110962);
    assert!(coordinates.latitude() > 0.0000037878 && coordinates.latitude() < 0.0000037880);
    assert!(coordinates.distance() > 0.98307506 && coordinates.distance() < 0.98307582);

    let coordinates = vsop87d::earth(2195870.0);

    assert!(coordinates.longitude() > 1.6622048656 && coordinates.longitude() < 1.6622048658);
    assert!(coordinates.latitude() > 0.0000015132 && coordinates.latitude() < 0.0000015134);
    assert!(coordinates.distance() > 0.98309386 && coordinates.distance() < 0.98309462);

    let coordinates = vsop87d::earth(2159345.0);

    assert!(coordinates.longitude() > 1.6495143196 && coordinates.longitude() < 1.6495143198);
    assert!(coordinates.latitude() > -0.0000013004 && coordinates.latitude() < -0.0000013002);
    assert!(coordinates.distance() > 0.98304366 && coordinates.distance() < 0.98304442);

    let coordinates = vsop87d::earth(2122820.0);

    assert!(coordinates.longitude() > 1.6367193622 && coordinates.longitude() < 1.6367193624);
    assert!(coordinates.latitude() > -0.0000031293 && coordinates.latitude() < -0.0000031291);
    assert!(coordinates.distance() > 0.98303280 && coordinates.distance() < 0.98303356);
}

#[test]
fn it_mars() {
    let coordinates = vsop87d::mars(2451545.0);

    assert!(coordinates.longitude() > 6.2735389982 && coordinates.longitude() < 6.2735389984);
    assert!(coordinates.latitude() > -0.0247779825 && coordinates.latitude() < -0.0247779823);
    assert!(coordinates.distance() > 1.39120731 && coordinates.distance() < 1.39120807);

    let coordinates = vsop87d::mars(2415020.0);

    assert!(coordinates.longitude() > 4.9942005210 && coordinates.longitude() < 4.9942005212);
    assert!(coordinates.latitude() > -0.0271965870 && coordinates.latitude() < -0.0271965868);
    assert!(coordinates.distance() > 1.42187739 && coordinates.distance() < 1.42187815);

    let coordinates = vsop87d::mars(2378495.0);

    assert!(coordinates.longitude() > 3.8711855477 && coordinates.longitude() < 3.8711855479);
    assert!(coordinates.latitude() > 0.0034969938 && coordinates.latitude() < 0.0034969940);
    assert!(coordinates.distance() > 1.56151362 && coordinates.distance() < 1.56151438);

    let coordinates = vsop87d::mars(2341970.0);

    assert!(coordinates.longitude() > 2.9166648689 && coordinates.longitude() < 2.9166648691);
    assert!(coordinates.latitude() > 0.0280268148 && coordinates.latitude() < 0.0280268150);
    assert!(coordinates.distance() > 1.65846933 && coordinates.distance() < 1.65847009);

    let coordinates = vsop87d::mars(2305445.0);

    assert!(coordinates.longitude() > 2.0058210393 && coordinates.longitude() < 2.0058210395);
    assert!(coordinates.latitude() > 0.0300702180 && coordinates.latitude() < 0.0300702182);
    assert!(coordinates.distance() > 1.63719934 && coordinates.distance() < 1.63720010);

    let coordinates = vsop87d::mars(2268920.0);

    assert!(coordinates.longitude() > 1.0050966938 && coordinates.longitude() < 1.0050966940);
    assert!(coordinates.latitude() > 0.0066676097 && coordinates.latitude() < 0.0066676099);
    assert!(coordinates.distance() > 1.51236189 && coordinates.distance() < 1.51236265);

    let coordinates = vsop87d::mars(2232395.0);

    assert!(coordinates.longitude() > 6.0979760761 && coordinates.longitude() < 6.0979760763);
    assert!(coordinates.latitude() > -0.0266794244 && coordinates.latitude() < -0.0266794242);
    assert!(coordinates.distance() > 1.39259607 && coordinates.distance() < 1.39259683);

    let coordinates = vsop87d::mars(2195870.0);

    assert!(coordinates.longitude() > 4.8193924947 && coordinates.longitude() < 4.8193924949);
    assert!(coordinates.latitude() > -0.0255031924 && coordinates.latitude() < -0.0255031922);
    assert!(coordinates.distance() > 1.42087034 && coordinates.distance() < 1.42087110);

    let coordinates = vsop87d::mars(2159345.0);

    assert!(coordinates.longitude() > 3.6939294874 && coordinates.longitude() < 3.6939294876);
    assert!(coordinates.latitude() > 0.0065885508 && coordinates.latitude() < 0.0065885510);
    assert!(coordinates.distance() > 1.55937982 && coordinates.distance() < 1.55938058);

    let coordinates = vsop87d::mars(2122820.0);

    assert!(coordinates.longitude() > 2.7367104343 && coordinates.longitude() < 2.7367104345);
    assert!(coordinates.latitude() > 0.0295522718 && coordinates.latitude() < 0.0295522720);
    assert!(coordinates.distance() > 1.65709985 && coordinates.distance() < 1.65710061);
}

#[test]
fn it_jupiter() {
    let coordinates = vsop87d::jupiter(2451545.0);

    assert!(coordinates.longitude() > 0.6334614185 && coordinates.longitude() < 0.6334614187);
    assert!(coordinates.latitude() > -0.0205001040 && coordinates.latitude() < -0.0205001038);
    assert!(coordinates.distance() > 4.96538094 && coordinates.distance() < 4.96538170);

    let coordinates = vsop87d::jupiter(2415020.0);

    assert!(coordinates.longitude() > 4.0927527023 && coordinates.longitude() < 4.0927527025);
    assert!(coordinates.latitude() > 0.0161446617 && coordinates.latitude() < 0.0161446619);
    assert!(coordinates.distance() > 5.38502729 && coordinates.distance() < 5.38502805);

    let coordinates = vsop87d::jupiter(2378495.0);

    assert!(coordinates.longitude() > 1.5255696770 && coordinates.longitude() < 1.5255696772);
    assert!(coordinates.latitude() > -0.0043606937 && coordinates.latitude() < -0.0043606935);
    assert!(coordinates.distance() > 5.13184538 && coordinates.distance() < 5.13184614);

    let coordinates = vsop87d::jupiter(2341970.0);

    assert!(coordinates.longitude() > 4.8888943124 && coordinates.longitude() < 4.8888943126);
    assert!(coordinates.latitude() > -0.0011098086 && coordinates.latitude() < -0.0011098084);
    assert!(coordinates.distance() > 5.18881299 && coordinates.distance() < 5.18881375);

    let coordinates = vsop87d::jupiter(2305445.0);

    assert!(coordinates.longitude() > 2.3348832683 && coordinates.longitude() < 2.3348832685);
    assert!(coordinates.latitude() > 0.0140523906 && coordinates.latitude() < 0.0140523908);
    assert!(coordinates.distance() > 5.34394512 && coordinates.distance() < 5.34394588);

    let coordinates = vsop87d::jupiter(2268920.0);

    assert!(coordinates.longitude() > 5.7527666851 && coordinates.longitude() < 5.7527666853);
    assert!(coordinates.latitude() > -0.0188346312 && coordinates.latitude() < -0.0188346310);
    assert!(coordinates.distance() > 5.00180036 && coordinates.distance() < 5.00180112);

    let coordinates = vsop87d::jupiter(2232395.0);

    assert!(coordinates.longitude() > 3.0889515349 && coordinates.longitude() < 3.0889515351);
    assert!(coordinates.latitude() > 0.0231157946 && coordinates.latitude() < 0.0231157948);
    assert!(coordinates.distance() > 5.44915664 && coordinates.distance() < 5.44915740);

    let coordinates = vsop87d::jupiter(2195870.0);

    assert!(coordinates.longitude() > 0.3776503429 && coordinates.longitude() < 0.3776503431);
    assert!(coordinates.latitude() > -0.0222448937 && coordinates.latitude() < -0.0222448935);
    assert!(coordinates.distance() > 4.97150672 && coordinates.distance() < 4.97150748);

    let coordinates = vsop87d::jupiter(2159345.0);

    assert!(coordinates.longitude() > 3.8455069136 && coordinates.longitude() < 3.8455069138);
    assert!(coordinates.latitude() > 0.0185554472 && coordinates.latitude() < 0.0185554474);
    assert!(coordinates.distance() > 5.38962031 && coordinates.distance() < 5.38962107);

    let coordinates = vsop87d::jupiter(2122820.0);

    assert!(coordinates.longitude() > 1.2695066545 && coordinates.longitude() < 1.2695066547);
    assert!(coordinates.latitude() > -0.0075335741 && coordinates.latitude() < -0.0075335739);
    assert!(coordinates.distance() > 5.11935836 && coordinates.distance() < 5.11935912);
}

#[test]
fn it_saturn() {
    let coordinates = vsop87d::saturn(2451545.0);

    assert!(coordinates.longitude() > 0.7980038760 && coordinates.longitude() < 0.7980038762);
    assert!(coordinates.latitude() > -0.0401984150 && coordinates.latitude() < -0.0401984148);
    assert!(coordinates.distance() > 9.18384799 && coordinates.distance() < 9.18384875);

    let coordinates = vsop87d::saturn(2415020.0);

    assert!(coordinates.longitude() > 4.6512836346 && coordinates.longitude() < 4.6512836348);
    assert!(coordinates.latitude() > 0.0192701408 && coordinates.latitude() < 0.0192701410);
    assert!(coordinates.distance() > 10.06685282 && coordinates.distance() < 10.06685358);

    let coordinates = vsop87d::saturn(2378495.0);

    assert!(coordinates.longitude() > 2.1956677358 && coordinates.longitude() < 2.1956677360);
    assert!(coordinates.latitude() > 0.0104156565 && coordinates.latitude() < 0.0104156567);
    assert!(coordinates.distance() > 9.10430648 && coordinates.distance() < 9.10430724);

    let coordinates = vsop87d::saturn(2341970.0);

    assert!(coordinates.longitude() > 5.8113963636 && coordinates.longitude() < 5.8113963638);
    assert!(coordinates.latitude() > -0.0291472788 && coordinates.latitude() < -0.0291472786);
    assert!(coordinates.distance() > 9.76299911 && coordinates.distance() < 9.76299987);

    let coordinates = vsop87d::saturn(2305445.0);

    assert!(coordinates.longitude() > 3.5217555198 && coordinates.longitude() < 3.5217555200);
    assert!(coordinates.latitude() > 0.0437035057 && coordinates.latitude() < 0.0437035059);
    assert!(coordinates.distance() > 9.75710318 && coordinates.distance() < 9.75710394);

    let coordinates = vsop87d::saturn(2268920.0);

    assert!(coordinates.longitude() > 0.8594235307 && coordinates.longitude() < 0.8594235309);
    assert!(coordinates.latitude() > -0.0379350089 && coordinates.latitude() < -0.0379350087);
    assert!(coordinates.distance() > 9.06692090 && coordinates.distance() < 9.06692166);

    let coordinates = vsop87d::saturn(2232395.0);

    assert!(coordinates.longitude() > 4.6913199263 && coordinates.longitude() < 4.6913199265);
    assert!(coordinates.latitude() > 0.0146771897 && coordinates.latitude() < 0.0146771899);
    assert!(coordinates.distance() > 10.10656892 && coordinates.distance() < 10.10656968);

    let coordinates = vsop87d::saturn(2195870.0);

    assert!(coordinates.longitude() > 2.2948875822 && coordinates.longitude() < 2.2948875824);
    assert!(coordinates.latitude() > 0.0178533696 && coordinates.latitude() < 0.0178533698);
    assert!(coordinates.distance() > 9.18575957 && coordinates.distance() < 9.18576033);

    let coordinates = vsop87d::saturn(2159345.0);

    assert!(coordinates.longitude() > 5.8660241563 && coordinates.longitude() < 5.8660241565);
    assert!(coordinates.latitude() > -0.0333866504 && coordinates.latitude() < -0.0333866502);
    assert!(coordinates.distance() > 9.59271701 && coordinates.distance() < 9.59271777);

    let coordinates = vsop87d::saturn(2122820.0);

    assert!(coordinates.longitude() > 3.5570108068 && coordinates.longitude() < 3.5570108070);
    assert!(coordinates.latitude() > 0.0435371138 && coordinates.latitude() < 0.0435371140);
    assert!(coordinates.distance() > 9.86699357 && coordinates.distance() < 9.86699433);
}

#[test]
fn it_uranus() {
    let coordinates = vsop87d::uranus(2451545.0);

    assert!(coordinates.longitude() > 5.5225485802 && coordinates.longitude() < 5.5225485804);
    assert!(coordinates.latitude() > -0.0119527839 && coordinates.latitude() < -0.0119527837);
    assert!(coordinates.distance() > 19.92404789 && coordinates.distance() < 19.92404865);

    let coordinates = vsop87d::uranus(2415020.0);

    assert!(coordinates.longitude() > 4.3397761172 && coordinates.longitude() < 4.3397761174);
    assert!(coordinates.latitude() > 0.0011570306 && coordinates.latitude() < 0.0011570308);
    assert!(coordinates.distance() > 18.99271598 && coordinates.distance() < 18.99271674);

    let coordinates = vsop87d::uranus(2378495.0);

    assert!(coordinates.longitude() > 3.0388348557 && coordinates.longitude() < 3.0388348559);
    assert!(coordinates.latitude() > 0.0132392954 && coordinates.latitude() < 0.0132392956);
    assert!(coordinates.distance() > 18.29911506 && coordinates.distance() < 18.29911582);

    let coordinates = vsop87d::uranus(2341970.0);

    assert!(coordinates.longitude() > 1.7242204719 && coordinates.longitude() < 1.7242204721);
    assert!(coordinates.latitude() > 0.0059836564 && coordinates.latitude() < 0.0059836566);
    assert!(coordinates.distance() > 18.79662051 && coordinates.distance() < 18.79662127);

    let coordinates = vsop87d::uranus(2305445.0);

    assert!(coordinates.longitude() > 0.5223325213 && coordinates.longitude() < 0.5223325215);
    assert!(coordinates.latitude() > -0.0089983886 && coordinates.latitude() < -0.0089983884);
    assert!(coordinates.distance() > 19.78198789 && coordinates.distance() < 19.78198865);

    let coordinates = vsop87d::uranus(2268920.0);

    assert!(coordinates.longitude() > 5.6817615581 && coordinates.longitude() < 5.6817615583);
    assert!(coordinates.latitude() > -0.0129257255 && coordinates.latitude() < -0.0129257253);
    assert!(coordinates.distance() > 20.03004592 && coordinates.distance() < 20.03004668);

    let coordinates = vsop87d::uranus(2232395.0);

    assert!(coordinates.longitude() > 4.5254482962 && coordinates.longitude() < 4.5254482964);
    assert!(coordinates.latitude() > -0.0019303341 && coordinates.latitude() < -0.0019303339);
    assert!(coordinates.distance() > 19.26943073 && coordinates.distance() < 19.26943149);

    let coordinates = vsop87d::uranus(2195870.0);

    assert!(coordinates.longitude() > 3.2557221719 && coordinates.longitude() < 3.2557221721);
    assert!(coordinates.latitude() > 0.0120919638 && coordinates.latitude() < 0.0120919640);
    assert!(coordinates.distance() > 18.39482248 && coordinates.distance() < 18.39482324);

    let coordinates = vsop87d::uranus(2159345.0);

    assert!(coordinates.longitude() > 1.9333853934 && coordinates.longitude() < 1.9333853936);
    assert!(coordinates.latitude() > 0.0088045917 && coordinates.latitude() < 0.0088045919);
    assert!(coordinates.distance() > 18.58414975 && coordinates.distance() < 18.58415051);

    let coordinates = vsop87d::uranus(2122820.0);

    assert!(coordinates.longitude() > 0.7007226223 && coordinates.longitude() < 0.7007226225);
    assert!(coordinates.latitude() > -0.0065610612 && coordinates.latitude() < -0.0065610610);
    assert!(coordinates.distance() > 19.56120745 && coordinates.distance() < 19.56120821);
}

#[test]
fn it_neptune() {
    let coordinates = vsop87d::neptune(2451545.0);

    assert!(coordinates.longitude() > 5.3045629251 && coordinates.longitude() < 5.3045629253);
    assert!(coordinates.latitude() > 0.0042236788 && coordinates.latitude() < 0.0042236790);
    assert!(coordinates.distance() > 30.12053246 && coordinates.distance() < 30.12053322);

    let coordinates = vsop87d::neptune(2415020.0);

    assert!(coordinates.longitude() > 1.4956195224 && coordinates.longitude() < 1.4956195226);
    assert!(coordinates.latitude() > -0.0219610031 && coordinates.latitude() < -0.0219610029);
    assert!(coordinates.distance() > 29.87103413 && coordinates.distance() < 29.87103489);

    let coordinates = vsop87d::neptune(2378495.0);

    assert!(coordinates.longitude() > 3.9290537976 && coordinates.longitude() < 3.9290537978);
    assert!(coordinates.latitude() > 0.0310692111 && coordinates.latitude() < 0.0310692113);
    assert!(coordinates.distance() > 30.32091885 && coordinates.distance() < 30.32091961);

    let coordinates = vsop87d::neptune(2341970.0);

    assert!(coordinates.longitude() > 0.0815199678 && coordinates.longitude() < 0.0815199680);
    assert!(coordinates.latitude() > -0.0260752534 && coordinates.latitude() < -0.0260752532);
    assert!(coordinates.distance() > 29.86858567 && coordinates.distance() < 29.86858643);

    let coordinates = vsop87d::neptune(2305445.0);

    assert!(coordinates.longitude() > 2.5537079777 && coordinates.longitude() < 2.5537079779);
    assert!(coordinates.latitude() > 0.0102374009 && coordinates.latitude() < 0.0102374011);
    assert!(coordinates.distance() > 30.13601549 && coordinates.distance() < 30.13601625);

    let coordinates = vsop87d::neptune(2268920.0);

    assert!(coordinates.longitude() > 4.9678695784 && coordinates.longitude() < 4.9678695786);
    assert!(coordinates.latitude() > 0.0116907776 && coordinates.latitude() < 0.0116907778);
    assert!(coordinates.distance() > 30.17853464 && coordinates.distance() < 30.17853540);

    let coordinates = vsop87d::neptune(2232395.0);

    assert!(coordinates.longitude() > 1.1523661583 && coordinates.longitude() < 1.1523661585);
    assert!(coordinates.latitude() > -0.0273547726 && coordinates.latitude() < -0.0273547724);
    assert!(coordinates.distance() > 29.83260514 && coordinates.distance() < 29.83260590);

    let coordinates = vsop87d::neptune(2195870.0);

    assert!(coordinates.longitude() > 3.5930943432 && coordinates.longitude() < 3.5930943434);
    assert!(coordinates.latitude() > 0.0316878974 && coordinates.latitude() < 0.0316878976);
    assert!(coordinates.distance() > 30.31091112 && coordinates.distance() < 30.31091188);

    let coordinates = vsop87d::neptune(2159345.0);

    assert!(coordinates.longitude() > 6.0203596579 && coordinates.longitude() < 6.0203596581);
    assert!(coordinates.latitude() > -0.0215169843 && coordinates.latitude() < -0.0215169841);
    assert!(coordinates.distance() > 29.90655030 && coordinates.distance() < 29.90655106);

    let coordinates = vsop87d::neptune(2122820.0);

    assert!(coordinates.longitude() > 2.2124988266 && coordinates.longitude() < 2.2124988268);
    assert!(coordinates.latitude() > 0.0027498092 && coordinates.latitude() < 0.0027498094);
    assert!(coordinates.distance() > 30.06536898 && coordinates.distance() < 30.06536974);
}
