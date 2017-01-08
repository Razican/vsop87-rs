extern crate vsop87;
use vsop87::*;

#[test]
fn it_sun() {
    let coordinates = vsop87e::sun(2451545.0);

    assert!(coordinates.x > -0.0071415280 && coordinates.x < -0.0071415278);
    assert!(coordinates.y > -0.0027881716 && coordinates.y < -0.0027881714);
    assert!(coordinates.z > 0.0002041 && coordinates.z < 0.0002081);

    let coordinates = vsop87e::sun(2415020.0);

    assert!(coordinates.x > 0.0031876596 && coordinates.x < 0.0031876598);
    assert!(coordinates.y > 0.0063575995 && coordinates.y < 0.0063575997);
    assert!(coordinates.z > -0.0001057 && coordinates.z < -0.0001017);

    let coordinates = vsop87e::sun(2378495.0);

    assert!(coordinates.x > 0.0034889966 && coordinates.x < 0.0034889968);
    assert!(coordinates.y > -0.0058080487 && coordinates.y < -0.0058080485);
    assert!(coordinates.z > -0.0000696 && coordinates.z < -0.0000656);

    let coordinates = vsop87e::sun(2341970.0);

    assert!(coordinates.x > -0.0051304707 && coordinates.x < -0.0051304705);
    assert!(coordinates.y > 0.0048363848 && coordinates.y < 0.0048363850);
    assert!(coordinates.z > 0.0001232 && coordinates.z < 0.0001272);

    let coordinates = vsop87e::sun(2305445.0);

    assert!(coordinates.x > 0.0070102894 && coordinates.x < 0.0070102896);
    assert!(coordinates.y > -0.0032796816 && coordinates.y < -0.0032796814);
    assert!(coordinates.z > -0.0002061 && coordinates.z < -0.0002021);

    let coordinates = vsop87e::sun(2268920.0);

    assert!(coordinates.x > -0.0071641327 && coordinates.x < -0.0071641325);
    assert!(coordinates.y > 0.0015976454 && coordinates.y < 0.0015976456);
    assert!(coordinates.z > 0.0001803 && coordinates.z < 0.0001843);

    let coordinates = vsop87e::sun(2232395.0);

    assert!(coordinates.x > 0.0044315255 && coordinates.x < 0.0044315257);
    assert!(coordinates.y > 0.0026981129 && coordinates.y < 0.0026981131);
    assert!(coordinates.z > -0.0001165 && coordinates.z < -0.0001125);

    let coordinates = vsop87e::sun(2195870.0);

    assert!(coordinates.x > 0.0000452874 && coordinates.x < 0.0000452876);
    assert!(coordinates.y > -0.0029730971 && coordinates.y < -0.0029730969);
    assert!(coordinates.z > -0.0000075 && coordinates.z < -0.0000035);

    let coordinates = vsop87e::sun(2159345.0);

    assert!(coordinates.x > -0.0005833298 && coordinates.x < -0.0005833296);
    assert!(coordinates.y > 0.0040370496 && coordinates.y < 0.0040370498);
    assert!(coordinates.z > 0.0000276 && coordinates.z < 0.0000316);

    let coordinates = vsop87e::sun(2122820.0);

    assert!(coordinates.x > 0.0025292638 && coordinates.x < 0.0025292640);
    assert!(coordinates.y > -0.0048917911 && coordinates.y < -0.0048917909);
    assert!(coordinates.z > -0.0000964 && coordinates.z < -0.0000924);
}

#[test]
fn it_mercury() {
    let coordinates = vsop87e::mercury(2451545.0);

    assert!(coordinates.x > -0.1372349395 && coordinates.x < -0.1372349393);
    assert!(coordinates.y > -0.4500758423 && coordinates.y < -0.4500758421);
    assert!(coordinates.z > -0.0243942 && coordinates.z < -0.0243902);

    let coordinates = vsop87e::mercury(2415020.0);

    assert!(coordinates.x > -0.3865370328 && coordinates.x < -0.3865370326);
    assert!(coordinates.y > -0.1438666202 && coordinates.y < -0.1438666200);
    assert!(coordinates.z > 0.0235142 && coordinates.z < 0.0235182);

    let coordinates = vsop87e::mercury(2378495.0);

    assert!(coordinates.x > -0.1648675271 && coordinates.x < -0.1648675269);
    assert!(coordinates.y > 0.2677027661 && coordinates.y < 0.2677027663);
    assert!(coordinates.z > 0.0377407 && coordinates.z < 0.0377447);

    let coordinates = vsop87e::mercury(2341970.0);

    assert!(coordinates.x > 0.3205415661 && coordinates.x < 0.3205415663);
    assert!(coordinates.y > 0.0929229658 && coordinates.y < 0.0929229660);
    assert!(coordinates.z > -0.0228588 && coordinates.z < -0.0228548);

    let coordinates = vsop87e::mercury(2305445.0);

    assert!(coordinates.x > 0.2384150858 && coordinates.x < 0.2384150860);
    assert!(coordinates.y > -0.3652917798 && coordinates.y < -0.3652917796);
    assert!(coordinates.z > -0.0510649 && coordinates.z < -0.0510609);

    let coordinates = vsop87e::mercury(2268920.0);

    assert!(coordinates.x > -0.1567195725 && coordinates.x < -0.1567195723);
    assert!(coordinates.y > -0.4393733657 && coordinates.y < -0.4393733655);
    assert!(coordinates.z > -0.0216323 && coordinates.z < -0.0216283);

    let coordinates = vsop87e::mercury(2232395.0);

    assert!(coordinates.x > -0.3894336634 && coordinates.x < -0.3894336632);
    assert!(coordinates.y > -0.1261418633 && coordinates.y < -0.1261418631);
    assert!(coordinates.z > 0.0262179 && coordinates.z < 0.0262219);

    let coordinates = vsop87e::mercury(2195870.0);

    assert!(coordinates.x > -0.1453788217 && coordinates.x < -0.1453788215);
    assert!(coordinates.y > 0.2807838471 && coordinates.y < 0.2807838473);
    assert!(coordinates.z > 0.0365184 && coordinates.z < 0.0365224);

    let coordinates = vsop87e::mercury(2159345.0);

    assert!(coordinates.x > 0.3334927267 && coordinates.x < 0.3334927269);
    assert!(coordinates.y > 0.0695495585 && coordinates.y < 0.0695495587);
    assert!(coordinates.z > -0.0260386 && coordinates.z < -0.0260346);

    let coordinates = vsop87e::mercury(2122820.0);

    assert!(coordinates.x > 0.2171621773 && coordinates.x < 0.2171621775);
    assert!(coordinates.y > -0.3801214150 && coordinates.y < -0.3801214148);
    assert!(coordinates.z > -0.0504947 && coordinates.z < -0.0504907);
}

#[test]
fn it_venus() {
    let coordinates = vsop87e::venus(2451545.0);

    assert!(coordinates.x > -0.7254438062 && coordinates.x < -0.7254438060);
    assert!(coordinates.y > -0.0354427730 && coordinates.y < -0.0354427728);
    assert!(coordinates.z > 0.0412184 && coordinates.z < 0.0412224);

    let coordinates = vsop87e::venus(2415020.0);

    assert!(coordinates.x > 0.7003304924 && coordinates.x < 0.7003304926);
    assert!(coordinates.y > -0.1970055159 && coordinates.y < -0.1970055157);
    assert!(coordinates.z > -0.0431258 && coordinates.z < -0.0431218);

    let coordinates = vsop87e::venus(2378495.0);

    assert!(coordinates.x > -0.5948645228 && coordinates.x < -0.5948645226);
    assert!(coordinates.y > 0.3900421674 && coordinates.y < 0.3900421676);
    assert!(coordinates.z > 0.0397542 && coordinates.z < 0.0397582);

    let coordinates = vsop87e::venus(2341970.0);

    assert!(coordinates.x > 0.4479888576 && coordinates.x < 0.4479888578);
    assert!(coordinates.y > -0.5644057139 && coordinates.y < -0.5644057137);
    assert!(coordinates.z > -0.0334412 && coordinates.z < -0.0334372);

    let coordinates = vsop87e::venus(2305445.0);

    assert!(coordinates.x > -0.2431871347 && coordinates.x < -0.2431871345);
    assert!(coordinates.y > 0.6700058593 && coordinates.y < 0.6700058595);
    assert!(coordinates.z > 0.0227654 && coordinates.z < 0.0227694);

    let coordinates = vsop87e::venus(2268920.0);

    assert!(coordinates.x > 0.0356693142 && coordinates.x < 0.0356693144);
    assert!(coordinates.y > -0.7243868494 && coordinates.y < -0.7243868492);
    assert!(coordinates.z > -0.0112222 && coordinates.z < -0.0112182);

    let coordinates = vsop87e::venus(2232395.0);

    assert!(coordinates.x > 0.1979737052 && coordinates.x < 0.1979737054);
    assert!(coordinates.y > 0.6967549126 && coordinates.y < 0.6967549128);
    assert!(coordinates.z > -0.0030468 && coordinates.z < -0.0030428);

    let coordinates = vsop87e::venus(2195870.0);

    assert!(coordinates.x > -0.3829606679 && coordinates.x < -0.3829606677);
    assert!(coordinates.y > -0.6180606559 && coordinates.y < -0.6180606557);
    assert!(coordinates.z > 0.0150845 && coordinates.z < 0.0150885);

    let coordinates = vsop87e::venus(2159345.0);

    assert!(coordinates.x > 0.5637717280 && coordinates.x < 0.5637717282);
    assert!(coordinates.y > 0.4559764947 && coordinates.y < 0.4559764949);
    assert!(coordinates.z > -0.0276968 && coordinates.z < -0.0276928);

    let coordinates = vsop87e::venus(2122820.0);

    assert!(coordinates.x > -0.6634865824 && coordinates.x < -0.6634865822);
    assert!(coordinates.y > -0.2802510191 && coordinates.y < -0.2802510189);
    assert!(coordinates.z > 0.0356910 && coordinates.z < 0.0356950);
}

#[test]
fn it_earth() {
    let coordinates = vsop87e::earth(2451545.0);

    assert!(coordinates.x > -0.1842769827 && coordinates.x < -0.1842769825);
    assert!(coordinates.y > 0.9644534529 && coordinates.y < 0.9644534531);
    assert!(coordinates.z > 0.0002002 && coordinates.z < 0.0002042);

    let coordinates = vsop87e::earth(2415020.0);

    assert!(coordinates.x > -0.1851203047 && coordinates.x < -0.1851203045);
    assert!(coordinates.y > 0.9714264842 && coordinates.y < 0.9714264844);
    assert!(coordinates.z > 0.0001093 && coordinates.z < 0.0001133);

    let coordinates = vsop87e::earth(2378495.0);

    assert!(coordinates.x > -0.1959028025 && coordinates.x < -0.1959028023);
    assert!(coordinates.y > 0.9569893871 && coordinates.y < 0.9569893873);
    assert!(coordinates.z > 0.0003611 && coordinates.z < 0.0003651);

    let coordinates = vsop87e::earth(2341970.0);

    assert!(coordinates.x > -0.2155959338 && coordinates.x < -0.2155959336);
    assert!(coordinates.y > 0.9651943804 && coordinates.y < 0.9651943806);
    assert!(coordinates.z > 0.0007705 && coordinates.z < 0.0007745);

    let coordinates = vsop87e::earth(2305445.0);

    assert!(coordinates.x > -0.2144880005 && coordinates.x < -0.2144880003);
    assert!(coordinates.y > 0.9545686379 && coordinates.y < 0.9545686381);
    assert!(coordinates.z > 0.0006508 && coordinates.z < 0.0006548);

    let coordinates = vsop87e::earth(2268920.0);

    assert!(coordinates.x > -0.2396421488 && coordinates.x < -0.2396421486);
    assert!(coordinates.y > 0.9567952241 && coordinates.y < 0.9567952243);
    assert!(coordinates.z > 0.0012496 && coordinates.z < 0.0012536);

    let coordinates = vsop87e::earth(2232395.0);

    assert!(coordinates.x > -0.2390819119 && coordinates.x < -0.2390819117);
    assert!(coordinates.y > 0.9551354476 && coordinates.y < 0.9551354478);
    assert!(coordinates.z > 0.0011706 && coordinates.z < 0.0011746);

    let coordinates = vsop87e::earth(2195870.0);

    assert!(coordinates.x > -0.2544150450 && coordinates.x < -0.2544150448);
    assert!(coordinates.y > 0.9466173267 && coordinates.y < 0.9466173269);
    assert!(coordinates.z > 0.0014887 && coordinates.z < 0.0014927);

    let coordinates = vsop87e::earth(2159345.0);

    assert!(coordinates.x > -0.2660380502 && coordinates.x < -0.2660380500);
    assert!(coordinates.y > 0.9505604080 && coordinates.y < 0.9505604082);
    assert!(coordinates.z > 0.0017313 && coordinates.z < 0.0017353);

    let coordinates = vsop87e::earth(2122820.0);

    assert!(coordinates.x > -0.2737854148 && coordinates.x < -0.2737854146);
    assert!(coordinates.y > 0.9385067423 && coordinates.y < 0.9385067425);
    assert!(coordinates.z > 0.0018151 && coordinates.z < 0.0018191);
}

#[test]
fn it_mars() {
    let coordinates = vsop87e::mars(2451545.0);

    assert!(coordinates.x > 1.3835744052 && coordinates.x < 1.3835744054);
    assert!(coordinates.y > -0.0162038667 && coordinates.y < -0.0162038665);
    assert!(coordinates.z > -0.0342637 && coordinates.z < -0.0342597);

    let coordinates = vsop87e::mars(2415020.0);

    assert!(coordinates.x > 0.4316209100 && coordinates.x < 0.4316209102);
    assert!(coordinates.y > -1.3488778275 && coordinates.y < -1.3488778273);
    assert!(coordinates.z > -0.0390707 && coordinates.z < -0.0390667);

    let coordinates = vsop87e::mars(2378495.0);

    assert!(coordinates.x > -1.1084329673 && coordinates.x < -1.1084329671);
    assert!(coordinates.y > -1.1021343624 && coordinates.y < -1.1021343622);
    assert!(coordinates.z > 0.0048512 && coordinates.z < 0.0048552);

    let coordinates = vsop87e::mars(2341970.0);

    assert!(coordinates.x > -1.6438794160 && coordinates.x < -1.6438794158);
    assert!(coordinates.y > 0.2555469168 && coordinates.y < 0.2555469170);
    assert!(coordinates.z > 0.0466838 && coordinates.z < 0.0466878);

    let coordinates = vsop87e::mars(2305445.0);

    assert!(coordinates.x > -0.8237565239 && coordinates.x < -0.8237565237);
    assert!(coordinates.y > 1.4065798250 && coordinates.y < 1.4065798252);
    assert!(coordinates.z > 0.0502476 && coordinates.z < 0.0502516);

    let coordinates = vsop87e::mars(2268920.0);

    assert!(coordinates.x > 0.6423617627 && coordinates.x < 0.6423617629);
    assert!(coordinates.y > 1.3673278773 && coordinates.y < 1.3673278775);
    assert!(coordinates.z > 0.0118726 && coordinates.z < 0.0118766);

    let coordinates = vsop87e::mars(2232395.0);

    assert!(coordinates.x > 1.3954709857 && coordinates.x < 1.3954709859);
    assert!(coordinates.y > -0.0516858117 && coordinates.y < -0.0516858115);
    assert!(coordinates.z > -0.0372178 && coordinates.z < -0.0372138);

    let coordinates = vsop87e::mars(2195870.0);

    assert!(coordinates.x > 0.3890526769 && coordinates.x < 0.3890526771);
    assert!(coordinates.y > -1.3690161893 && coordinates.y < -1.3690161891);
    assert!(coordinates.z > -0.0383884 && coordinates.z < -0.0383844);

    let coordinates = vsop87e::mars(2159345.0);

    assert!(coordinates.x > -1.1446750446 && coordinates.x < -1.1446750444);
    assert!(coordinates.y > -1.0555162818 && coordinates.y < -1.0555162816);
    assert!(coordinates.z > 0.0082457 && coordinates.z < 0.0082497);

    let coordinates = vsop87e::mars(2122820.0);

    assert!(coordinates.x > -1.6253192612 && coordinates.x < -1.6253192610);
    assert!(coordinates.y > 0.3011276885 && coordinates.y < 0.3011276887);
    assert!(coordinates.z > 0.0493252 && coordinates.z < 0.0493292);
}

#[test]
fn it_jupiter() {
    let coordinates = vsop87e::jupiter(2451545.0);

    assert!(coordinates.x > 3.9940325024 && coordinates.x < 3.9940325026);
    assert!(coordinates.y > 2.9357928286 && coordinates.y < 2.9357928288);
    assert!(coordinates.z > -0.1015796 && coordinates.z < -0.1015756);

    let coordinates = vsop87e::jupiter(2415020.0);

    assert!(coordinates.x > -3.0159347741 && coordinates.x < -3.0159347739);
    assert!(coordinates.y > -4.4518987730 && coordinates.y < -4.4518987728);
    assert!(coordinates.z > 0.0857585 && coordinates.z < 0.0857625);

    let coordinates = vsop87e::jupiter(2378495.0);

    assert!(coordinates.x > -0.0145500118 && coordinates.x < -0.0145500116);
    assert!(coordinates.y > 5.1259668252 && coordinates.y < 5.1259668254);
    assert!(coordinates.z > -0.0201145 && coordinates.z < -0.0201105);

    let coordinates = vsop87e::jupiter(2341970.0);

    assert!(coordinates.x > 1.2766013836 && coordinates.x < 1.2766013838);
    assert!(coordinates.y > -5.0231716008 && coordinates.y < -5.0231716006);
    assert!(coordinates.z > -0.0090020 && coordinates.z < -0.0089980);

    let coordinates = vsop87e::jupiter(2305445.0);

    assert!(coordinates.x > -4.0477856740 && coordinates.x < -4.0477856738);
    assert!(coordinates.y > 3.4767059970 && coordinates.y < 3.4767059972);
    assert!(coordinates.z > 0.0777900 && coordinates.z < 0.0777940);

    let coordinates = vsop87e::jupiter(2268920.0);

    assert!(coordinates.x > 4.5819830418 && coordinates.x < 4.5819830420);
    assert!(coordinates.y > -1.9854861384 && coordinates.y < -1.9854861382);
    assert!(coordinates.z > -0.0959289 && coordinates.z < -0.0959249);

    let coordinates = vsop87e::jupiter(2232395.0);

    assert!(coordinates.x > -5.4195080605 && coordinates.x < -5.4195080603);
    assert!(coordinates.y > -0.5058506155 && coordinates.y < -0.5058506153);
    assert!(coordinates.z > 0.1246595 && coordinates.z < 0.1246635);

    let coordinates = vsop87e::jupiter(2195870.0);

    assert!(coordinates.x > 4.2423739219 && coordinates.x < 4.2423739221);
    assert!(coordinates.y > 2.5868702422 && coordinates.y < 2.5868702424);
    assert!(coordinates.z > -0.1060383 && coordinates.z < -0.1060343);

    let coordinates = vsop87e::jupiter(2159345.0);

    assert!(coordinates.x > -3.3560639243 && coordinates.x < -3.3560639241);
    assert!(coordinates.y > -4.2126331636 && coordinates.y < -4.2126331634);
    assert!(coordinates.z > 0.0919694 && coordinates.z < 0.0919734);

    let coordinates = vsop87e::jupiter(2122820.0);

    assert!(coordinates.x > 0.4233154539 && coordinates.x < 0.4233154541);
    assert!(coordinates.y > 5.0970673237 && coordinates.y < 5.0970673239);
    assert!(coordinates.z > -0.0281051 && coordinates.z < -0.0281011);
}

#[test]
fn it_saturn() {
    let coordinates = vsop87e::saturn(2451545.0);

    assert!(coordinates.x > 6.3992653458 && coordinates.x < 6.3992653460);
    assert!(coordinates.y > 6.5672047373 && coordinates.y < 6.5672047375);
    assert!(coordinates.z > -0.3688726 && coordinates.z < -0.3688686);

    let coordinates = vsop87e::saturn(2415020.0);

    assert!(coordinates.x > -0.3664097225 && coordinates.x < -0.3664097223);
    assert!(coordinates.y > -10.0518822110 && coordinates.y < -10.0518822108);
    assert!(coordinates.z > 0.1915798 && coordinates.z < 0.1915838);

    let coordinates = vsop87e::saturn(2378495.0);

    assert!(coordinates.x > -5.6756021086 && coordinates.x < -5.6756021084);
    assert!(coordinates.y > 7.1094397728 && coordinates.y < 7.1094397730);
    assert!(coordinates.z > 0.0977825 && coordinates.z < 0.0977865);

    let coordinates = vsop87e::saturn(2341970.0);

    assert!(coordinates.x > 8.9883454292 && coordinates.x < 8.9883454294);
    assert!(coordinates.y > -3.7834861556 && coordinates.y < -3.7834861554);
    assert!(coordinates.z > -0.2865158 && coordinates.z < -0.2865118);

    let coordinates = vsop87e::saturn(2305445.0);

    assert!(coordinates.x > -8.6500173349 && coordinates.x < -8.6500173347);
    assert!(coordinates.y > -4.4842589407 && coordinates.y < -4.4842589405);
    assert!(coordinates.z > 0.4214191 && coordinates.z < 0.4214231);

    let coordinates = vsop87e::saturn(2268920.0);

    assert!(coordinates.x > 5.0306933582 && coordinates.x < 5.0306933584);
    assert!(coordinates.y > 7.5326602150 && coordinates.y < 7.5326602152);
    assert!(coordinates.z > -0.3347077 && coordinates.z < -0.3347037);

    let coordinates = vsop87e::saturn(2232395.0);

    assert!(coordinates.x > 1.2645936161 && coordinates.x < 1.2645936163);
    assert!(coordinates.y > -10.0240954526 && coordinates.y < -10.0240954524);
    assert!(coordinates.z > 0.1345888 && coordinates.z < 0.1345928);

    let coordinates = vsop87e::saturn(2195870.0);

    assert!(coordinates.x > -7.1627672768 && coordinates.x < -7.1627672766);
    assert!(coordinates.y > 5.7452916043 && coordinates.y < 5.7452916045);
    assert!(coordinates.z > 0.1724887 && coordinates.z < 0.1724927);

    let coordinates = vsop87e::saturn(2159345.0);

    assert!(coordinates.x > 9.3505836059 && coordinates.x < 9.3505836061);
    assert!(coordinates.y > -2.1105535821 && coordinates.y < -2.1105535819);
    assert!(coordinates.z > -0.3230966 && coordinates.z < -0.3230926);

    let coordinates = vsop87e::saturn(2122820.0);

    assert!(coordinates.x > -7.9370266648 && coordinates.x < -7.9370266646);
    assert!(coordinates.y > -5.8484785040 && coordinates.y < -5.8484785038);
    assert!(coordinates.z > 0.4164638 && coordinates.z < 0.4164678);
}

#[test]
fn it_uranus() {
    let coordinates = vsop87e::uranus(2451545.0);

    assert!(coordinates.x > 14.4247519567 && coordinates.x < 14.4247519569);
    assert!(coordinates.y > -13.7371045088 && coordinates.y < -13.7371045086);
    assert!(coordinates.z > -0.2379381 && coordinates.z < -0.2379341);

    let coordinates = vsop87e::uranus(2415020.0);

    assert!(coordinates.x > -6.4778956414 && coordinates.x < -6.4778956412);
    assert!(coordinates.y > -17.8463318323 && coordinates.y < -17.8463318321);
    assert!(coordinates.z > 0.0176878 && coordinates.z < 0.0176918);

    let coordinates = vsop87e::uranus(2378495.0);

    assert!(coordinates.x > -18.2673444261 && coordinates.x < -18.2673444259);
    assert!(coordinates.y > 0.9819574798 && coordinates.y < 0.9819574800);
    assert!(coordinates.z > 0.2419649 && coordinates.z < 0.2419689);

    let coordinates = vsop87e::uranus(2341970.0);

    assert!(coordinates.x > -4.2265696171 && coordinates.x < -4.2265696169);
    assert!(coordinates.y > 18.3208630948 && coordinates.y < 18.3208630950);
    assert!(coordinates.z > 0.1248825 && coordinates.z < 0.1248865);

    let coordinates = vsop87e::uranus(2305445.0);

    assert!(coordinates.x > 16.1091090967 && coordinates.x < 16.1091090969);
    assert!(coordinates.y > 11.4867931079 && coordinates.y < 11.4867931081);
    assert!(coordinates.z > -0.1666680 && coordinates.z < -0.1666640);

    let coordinates = vsop87e::uranus(2268920.0);

    assert!(coordinates.x > 17.7611606375 && coordinates.x < 17.7611606377);
    assert!(coordinates.y > -9.2405618854 && coordinates.y < -9.2405618852);
    assert!(coordinates.z > -0.2678679 && coordinates.z < -0.2678639);

    let coordinates = vsop87e::uranus(2232395.0);

    assert!(coordinates.x > -0.7823849125 && coordinates.x < -0.7823849123);
    assert!(coordinates.y > -19.2505578585 && coordinates.y < -19.2505578583);
    assert!(coordinates.z > -0.0637589 && coordinates.z < -0.0637549);

    let coordinates = vsop87e::uranus(2195870.0);

    assert!(coordinates.x > -17.6538791198 && coordinates.x < -17.6538791196);
    assert!(coordinates.y > -5.1666300881 && coordinates.y < -5.1666300879);
    assert!(coordinates.z > 0.2124594 && coordinates.z < 0.2124634);

    let coordinates = vsop87e::uranus(2159345.0);

    assert!(coordinates.x > -9.8292937631 && coordinates.x < -9.8292937629);
    assert!(coordinates.y > 15.7752257094 && coordinates.y < 15.7752257096);
    assert!(coordinates.z > 0.1915092 && coordinates.z < 0.1915132);

    let coordinates = vsop87e::uranus(2122820.0);

    assert!(coordinates.x > 11.8571754272 && coordinates.x < 11.8571754274);
    assert!(coordinates.y > 15.5546452201 && coordinates.y < 15.5546452203);
    assert!(coordinates.z > -0.0951153 && coordinates.z < -0.0951113);
}

#[test]
fn it_neptune() {
    let coordinates = vsop87e::neptune(2451545.0);

    assert!(coordinates.x > 16.8049701268 && coordinates.x < 16.8049701270);
    assert!(coordinates.y > -24.9944513570 && coordinates.y < -24.9944513568);
    assert!(coordinates.z > 0.1274231 && coordinates.z < 0.1274271);

    let coordinates = vsop87e::neptune(2415020.0);

    assert!(coordinates.x > 1.5196434116 && coordinates.x < 1.5196434118);
    assert!(coordinates.y > 29.8318114918 && coordinates.y < 29.8318114920);
    assert!(coordinates.z > -0.6492457 && coordinates.z < -0.6492417);

    let coordinates = vsop87e::neptune(2378495.0);

    assert!(coordinates.x > -20.3104052893 && coordinates.x < -20.3104052891);
    assert!(coordinates.y > -22.4966336301 && coordinates.y < -22.4966336299);
    assert!(coordinates.z > 0.9308456 && coordinates.z < 0.9308496);

    let coordinates = vsop87e::neptune(2341970.0);

    assert!(coordinates.x > 29.4971507219 && coordinates.x < 29.4971507221);
    assert!(coordinates.y > 4.6036063556 && coordinates.y < 4.6036063558);
    assert!(coordinates.z > -0.7739275 && coordinates.z < -0.7739235);

    let coordinates = vsop87e::neptune(2305445.0);

    assert!(coordinates.x > -26.5753161425 && coordinates.x < -26.5753161423);
    assert!(coordinates.y > 14.1902813793 && coordinates.y < 14.1902813795);
    assert!(coordinates.z > 0.3194782 && coordinates.z < 0.3194822);

    let coordinates = vsop87e::neptune(2268920.0);

    assert!(coordinates.x > 11.1089045477 && coordinates.x < 11.1089045479);
    assert!(coordinates.y > -28.0532332602 && coordinates.y < -28.0532332600);
    assert!(coordinates.z > 0.3218536 && coordinates.z < 0.3218576);

    let coordinates = vsop87e::neptune(2232395.0);

    assert!(coordinates.x > 8.0258639137 && coordinates.x < 8.0258639139);
    assert!(coordinates.y > 28.7261896413 && coordinates.y < 28.7261896415);
    assert!(coordinates.z > -0.7760224 && coordinates.z < -0.7760184);

    let coordinates = vsop87e::neptune(2195870.0);

    assert!(coordinates.x > -24.6233894508 && coordinates.x < -24.6233894506);
    assert!(coordinates.y > -17.6544160440 && coordinates.y < -17.6544160438);
    assert!(coordinates.z > 0.9297169 && coordinates.z < 0.9297209);

    let coordinates = vsop87e::neptune(2159345.0);

    assert!(coordinates.x > 29.8297729718 && coordinates.x < 29.8297729720);
    assert!(coordinates.y > -2.0298541973 && coordinates.y < -2.0298541971);
    assert!(coordinates.z > -0.6440972 && coordinates.z < -0.6440932);

    let coordinates = vsop87e::neptune(2122820.0);

    assert!(coordinates.x > -22.7959876639 && coordinates.x < -22.7959876637);
    assert!(coordinates.y > 19.5945850297 && coordinates.y < 19.5945850299);
    assert!(coordinates.z > 0.1205410 && coordinates.z < 0.1205450);
}
