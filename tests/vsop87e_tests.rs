extern crate vsop87;
use vsop87::*;

#[test]
fn it_sun() {
    let (x, y, z) = vsop87e::sun(2451545.0);

    assert!(x > -0.0071415280 && x < -0.0071415278);
    assert!(y > -0.0027881716 && y < -0.0027881714);
    assert!(z > 0.0002041 && z < 0.0002081);

    let (x, y, z) = vsop87e::sun(2415020.0);

    assert!(x > 0.0031876596 && x < 0.0031876598);
    assert!(y > 0.0063575995 && y < 0.0063575997);
    assert!(z > -0.0001057 && z < -0.0001017);

    let (x, y, z) = vsop87e::sun(2378495.0);

    assert!(x > 0.0034889966 && x < 0.0034889968);
    assert!(y > -0.0058080487 && y < -0.0058080485);
    assert!(z > -0.0000696 && z < -0.0000656);

    let (x, y, z) = vsop87e::sun(2341970.0);

    assert!(x > -0.0051304707 && x < -0.0051304705);
    assert!(y > 0.0048363848 && y < 0.0048363850);
    assert!(z > 0.0001232 && z < 0.0001272);

    let (x, y, z) = vsop87e::sun(2305445.0);

    assert!(x > 0.0070102894 && x < 0.0070102896);
    assert!(y > -0.0032796816 && y < -0.0032796814);
    assert!(z > -0.0002061 && z < -0.0002021);

    let (x, y, z) = vsop87e::sun(2268920.0);

    assert!(x > -0.0071641327 && x < -0.0071641325);
    assert!(y > 0.0015976454 && y < 0.0015976456);
    assert!(z > 0.0001803 && z < 0.0001843);

    let (x, y, z) = vsop87e::sun(2232395.0);

    assert!(x > 0.0044315255 && x < 0.0044315257);
    assert!(y > 0.0026981129 && y < 0.0026981131);
    assert!(z > -0.0001165 && z < -0.0001125);

    let (x, y, z) = vsop87e::sun(2195870.0);

    assert!(x > 0.0000452874 && x < 0.0000452876);
    assert!(y > -0.0029730971 && y < -0.0029730969);
    assert!(z > -0.0000075 && z < -0.0000035);

    let (x, y, z) = vsop87e::sun(2159345.0);

    assert!(x > -0.0005833298 && x < -0.0005833296);
    assert!(y > 0.0040370496 && y < 0.0040370498);
    assert!(z > 0.0000276 && z < 0.0000316);

    let (x, y, z) = vsop87e::sun(2122820.0);

    assert!(x > 0.0025292638 && x < 0.0025292640);
    assert!(y > -0.0048917911 && y < -0.0048917909);
    assert!(z > -0.0000964 && z < -0.0000924);
}

#[test]
fn it_mercury() {
    let (x, y, z) = vsop87e::mercury(2451545.0);

    assert!(x > -0.1372349395 && x < -0.1372349393);
    assert!(y > -0.4500758423 && y < -0.4500758421);
    assert!(z > -0.0243942 && z < -0.0243902);

    let (x, y, z) = vsop87e::mercury(2415020.0);

    assert!(x > -0.3865370328 && x < -0.3865370326);
    assert!(y > -0.1438666202 && y < -0.1438666200);
    assert!(z > 0.0235142 && z < 0.0235182);

    let (x, y, z) = vsop87e::mercury(2378495.0);

    assert!(x > -0.1648675271 && x < -0.1648675269);
    assert!(y > 0.2677027661 && y < 0.2677027663);
    assert!(z > 0.0377407 && z < 0.0377447);

    let (x, y, z) = vsop87e::mercury(2341970.0);

    assert!(x > 0.3205415661 && x < 0.3205415663);
    assert!(y > 0.0929229658 && y < 0.0929229660);
    assert!(z > -0.0228588 && z < -0.0228548);

    let (x, y, z) = vsop87e::mercury(2305445.0);

    assert!(x > 0.2384150858 && x < 0.2384150860);
    assert!(y > -0.3652917798 && y < -0.3652917796);
    assert!(z > -0.0510649 && z < -0.0510609);

    let (x, y, z) = vsop87e::mercury(2268920.0);

    assert!(x > -0.1567195725 && x < -0.1567195723);
    assert!(y > -0.4393733657 && y < -0.4393733655);
    assert!(z > -0.0216323 && z < -0.0216283);

    let (x, y, z) = vsop87e::mercury(2232395.0);

    assert!(x > -0.3894336634 && x < -0.3894336632);
    assert!(y > -0.1261418633 && y < -0.1261418631);
    assert!(z > 0.0262179 && z < 0.0262219);

    let (x, y, z) = vsop87e::mercury(2195870.0);

    assert!(x > -0.1453788217 && x < -0.1453788215);
    assert!(y > 0.2807838471 && y < 0.2807838473);
    assert!(z > 0.0365184 && z < 0.0365224);

    let (x, y, z) = vsop87e::mercury(2159345.0);

    assert!(x > 0.3334927267 && x < 0.3334927269);
    assert!(y > 0.0695495585 && y < 0.0695495587);
    assert!(z > -0.0260386 && z < -0.0260346);

    let (x, y, z) = vsop87e::mercury(2122820.0);

    assert!(x > 0.2171621773 && x < 0.2171621775);
    assert!(y > -0.3801214150 && y < -0.3801214148);
    assert!(z > -0.0504947 && z < -0.0504907);
}

#[test]
fn it_venus() {
    let (x, y, z) = vsop87e::venus(2451545.0);

    assert!(x > -0.7254438062 && x < -0.7254438060);
    assert!(y > -0.0354427730 && y < -0.0354427728);
    assert!(z > 0.0412184 && z < 0.0412224);

    let (x, y, z) = vsop87e::venus(2415020.0);

    assert!(x > 0.7003304924 && x < 0.7003304926);
    assert!(y > -0.1970055159 && y < -0.1970055157);
    assert!(z > -0.0431258 && z < -0.0431218);

    let (x, y, z) = vsop87e::venus(2378495.0);

    assert!(x > -0.5948645228 && x < -0.5948645226);
    assert!(y > 0.3900421674 && y < 0.3900421676);
    assert!(z > 0.0397542 && z < 0.0397582);

    let (x, y, z) = vsop87e::venus(2341970.0);

    assert!(x > 0.4479888576 && x < 0.4479888578);
    assert!(y > -0.5644057139 && y < -0.5644057137);
    assert!(z > -0.0334412 && z < -0.0334372);

    let (x, y, z) = vsop87e::venus(2305445.0);

    assert!(x > -0.2431871347 && x < -0.2431871345);
    assert!(y > 0.6700058593 && y < 0.6700058595);
    assert!(z > 0.0227654 && z < 0.0227694);

    let (x, y, z) = vsop87e::venus(2268920.0);

    assert!(x > 0.0356693142 && x < 0.0356693144);
    assert!(y > -0.7243868494 && y < -0.7243868492);
    assert!(z > -0.0112222 && z < -0.0112182);

    let (x, y, z) = vsop87e::venus(2232395.0);

    assert!(x > 0.1979737052 && x < 0.1979737054);
    assert!(y > 0.6967549126 && y < 0.6967549128);
    assert!(z > -0.0030468 && z < -0.0030428);

    let (x, y, z) = vsop87e::venus(2195870.0);

    assert!(x > -0.3829606679 && x < -0.3829606677);
    assert!(y > -0.6180606559 && y < -0.6180606557);
    assert!(z > 0.0150845 && z < 0.0150885);

    let (x, y, z) = vsop87e::venus(2159345.0);

    assert!(x > 0.5637717280 && x < 0.5637717282);
    assert!(y > 0.4559764947 && y < 0.4559764949);
    assert!(z > -0.0276968 && z < -0.0276928);

    let (x, y, z) = vsop87e::venus(2122820.0);

    assert!(x > -0.6634865824 && x < -0.6634865822);
    assert!(y > -0.2802510191 && y < -0.2802510189);
    assert!(z > 0.0356910 && z < 0.0356950);
}

#[test]
fn it_earth() {
    let (x, y, z) = vsop87e::earth(2451545.0);

    assert!(x > -0.1842769827 && x < -0.1842769825);
    assert!(y > 0.9644534529 && y < 0.9644534531);
    assert!(z > 0.0002002 && z < 0.0002042);

    let (x, y, z) = vsop87e::earth(2415020.0);

    assert!(x > -0.1851203047 && x < -0.1851203045);
    assert!(y > 0.9714264842 && y < 0.9714264844);
    assert!(z > 0.0001093 && z < 0.0001133);

    let (x, y, z) = vsop87e::earth(2378495.0);

    assert!(x > -0.1959028025 && x < -0.1959028023);
    assert!(y > 0.9569893871 && y < 0.9569893873);
    assert!(z > 0.0003611 && z < 0.0003651);

    let (x, y, z) = vsop87e::earth(2341970.0);

    assert!(x > -0.2155959338 && x < -0.2155959336);
    assert!(y > 0.9651943804 && y < 0.9651943806);
    assert!(z > 0.0007705 && z < 0.0007745);

    let (x, y, z) = vsop87e::earth(2305445.0);

    assert!(x > -0.2144880005 && x < -0.2144880003);
    assert!(y > 0.9545686379 && y < 0.9545686381);
    assert!(z > 0.0006508 && z < 0.0006548);

    let (x, y, z) = vsop87e::earth(2268920.0);

    assert!(x > -0.2396421488 && x < -0.2396421486);
    assert!(y > 0.9567952241 && y < 0.9567952243);
    assert!(z > 0.0012496 && z < 0.0012536);

    let (x, y, z) = vsop87e::earth(2232395.0);

    assert!(x > -0.2390819119 && x < -0.2390819117);
    assert!(y > 0.9551354476 && y < 0.9551354478);
    assert!(z > 0.0011706 && z < 0.0011746);

    let (x, y, z) = vsop87e::earth(2195870.0);

    assert!(x > -0.2544150450 && x < -0.2544150448);
    assert!(y > 0.9466173267 && y < 0.9466173269);
    assert!(z > 0.0014887 && z < 0.0014927);

    let (x, y, z) = vsop87e::earth(2159345.0);

    assert!(x > -0.2660380502 && x < -0.2660380500);
    assert!(y > 0.9505604080 && y < 0.9505604082);
    assert!(z > 0.0017313 && z < 0.0017353);

    let (x, y, z) = vsop87e::earth(2122820.0);

    assert!(x > -0.2737854148 && x < -0.2737854146);
    assert!(y > 0.9385067423 && y < 0.9385067425);
    assert!(z > 0.0018151 && z < 0.0018191);
}

#[test]
fn it_mars() {
    let (x, y, z) = vsop87e::mars(2451545.0);

    assert!(x > 1.3835744052 && x < 1.3835744054);
    assert!(y > -0.0162038667 && y < -0.0162038665);
    assert!(z > -0.0342637 && z < -0.0342597);

    let (x, y, z) = vsop87e::mars(2415020.0);

    assert!(x > 0.4316209100 && x < 0.4316209102);
    assert!(y > -1.3488778275 && y < -1.3488778273);
    assert!(z > -0.0390707 && z < -0.0390667);

    let (x, y, z) = vsop87e::mars(2378495.0);

    assert!(x > -1.1084329673 && x < -1.1084329671);
    assert!(y > -1.1021343624 && y < -1.1021343622);
    assert!(z > 0.0048512 && z < 0.0048552);

    let (x, y, z) = vsop87e::mars(2341970.0);

    assert!(x > -1.6438794160 && x < -1.6438794158);
    assert!(y > 0.2555469168 && y < 0.2555469170);
    assert!(z > 0.0466838 && z < 0.0466878);

    let (x, y, z) = vsop87e::mars(2305445.0);

    assert!(x > -0.8237565239 && x < -0.8237565237);
    assert!(y > 1.4065798250 && y < 1.4065798252);
    assert!(z > 0.0502476 && z < 0.0502516);

    let (x, y, z) = vsop87e::mars(2268920.0);

    assert!(x > 0.6423617627 && x < 0.6423617629);
    assert!(y > 1.3673278773 && y < 1.3673278775);
    assert!(z > 0.0118726 && z < 0.0118766);

    let (x, y, z) = vsop87e::mars(2232395.0);

    assert!(x > 1.3954709857 && x < 1.3954709859);
    assert!(y > -0.0516858117 && y < -0.0516858115);
    assert!(z > -0.0372178 && z < -0.0372138);

    let (x, y, z) = vsop87e::mars(2195870.0);

    assert!(x > 0.3890526769 && x < 0.3890526771);
    assert!(y > -1.3690161893 && y < -1.3690161891);
    assert!(z > -0.0383884 && z < -0.0383844);

    let (x, y, z) = vsop87e::mars(2159345.0);

    assert!(x > -1.1446750446 && x < -1.1446750444);
    assert!(y > -1.0555162818 && y < -1.0555162816);
    assert!(z > 0.0082457 && z < 0.0082497);

    let (x, y, z) = vsop87e::mars(2122820.0);

    assert!(x > -1.6253192612 && x < -1.6253192610);
    assert!(y > 0.3011276885 && y < 0.3011276887);
    assert!(z > 0.0493252 && z < 0.0493292);
}

#[test]
fn it_jupiter() {
    let (x, y, z) = vsop87e::jupiter(2451545.0);

    assert!(x > 3.9940325024 && x < 3.9940325026);
    assert!(y > 2.9357928286 && y < 2.9357928288);
    assert!(z > -0.1015796 && z < -0.1015756);

    let (x, y, z) = vsop87e::jupiter(2415020.0);

    assert!(x > -3.0159347741 && x < -3.0159347739);
    assert!(y > -4.4518987730 && y < -4.4518987728);
    assert!(z > 0.0857585 && z < 0.0857625);

    let (x, y, z) = vsop87e::jupiter(2378495.0);

    assert!(x > -0.0145500118 && x < -0.0145500116);
    assert!(y > 5.1259668252 && y < 5.1259668254);
    assert!(z > -0.0201145 && z < -0.0201105);

    let (x, y, z) = vsop87e::jupiter(2341970.0);

    assert!(x > 1.2766013836 && x < 1.2766013838);
    assert!(y > -5.0231716008 && y < -5.0231716006);
    assert!(z > -0.0090020 && z < -0.0089980);

    let (x, y, z) = vsop87e::jupiter(2305445.0);

    assert!(x > -4.0477856740 && x < -4.0477856738);
    assert!(y > 3.4767059970 && y < 3.4767059972);
    assert!(z > 0.0777900 && z < 0.0777940);

    let (x, y, z) = vsop87e::jupiter(2268920.0);

    assert!(x > 4.5819830418 && x < 4.5819830420);
    assert!(y > -1.9854861384 && y < -1.9854861382);
    assert!(z > -0.0959289 && z < -0.0959249);

    let (x, y, z) = vsop87e::jupiter(2232395.0);

    assert!(x > -5.4195080605 && x < -5.4195080603);
    assert!(y > -0.5058506155 && y < -0.5058506153);
    assert!(z > 0.1246595 && z < 0.1246635);

    let (x, y, z) = vsop87e::jupiter(2195870.0);

    assert!(x > 4.2423739219 && x < 4.2423739221);
    assert!(y > 2.5868702422 && y < 2.5868702424);
    assert!(z > -0.1060383 && z < -0.1060343);

    let (x, y, z) = vsop87e::jupiter(2159345.0);

    assert!(x > -3.3560639243 && x < -3.3560639241);
    assert!(y > -4.2126331636 && y < -4.2126331634);
    assert!(z > 0.0919694 && z < 0.0919734);

    let (x, y, z) = vsop87e::jupiter(2122820.0);

    assert!(x > 0.4233154539 && x < 0.4233154541);
    assert!(y > 5.0970673237 && y < 5.0970673239);
    assert!(z > -0.0281051 && z < -0.0281011);
}

#[test]
fn it_saturn() {
    let (x, y, z) = vsop87e::saturn(2451545.0);

    assert!(x > 6.3992653458 && x < 6.3992653460);
    assert!(y > 6.5672047373 && y < 6.5672047375);
    assert!(z > -0.3688726 && z < -0.3688686);

    let (x, y, z) = vsop87e::saturn(2415020.0);

    assert!(x > -0.3664097225 && x < -0.3664097223);
    assert!(y > -10.0518822110 && y < -10.0518822108);
    assert!(z > 0.1915798 && z < 0.1915838);

    let (x, y, z) = vsop87e::saturn(2378495.0);

    assert!(x > -5.6756021086 && x < -5.6756021084);
    assert!(y > 7.1094397728 && y < 7.1094397730);
    assert!(z > 0.0977825 && z < 0.0977865);

    let (x, y, z) = vsop87e::saturn(2341970.0);

    assert!(x > 8.9883454292 && x < 8.9883454294);
    assert!(y > -3.7834861556 && y < -3.7834861554);
    assert!(z > -0.2865158 && z < -0.2865118);

    let (x, y, z) = vsop87e::saturn(2305445.0);

    assert!(x > -8.6500173349 && x < -8.6500173347);
    assert!(y > -4.4842589407 && y < -4.4842589405);
    assert!(z > 0.4214191 && z < 0.4214231);

    let (x, y, z) = vsop87e::saturn(2268920.0);

    assert!(x > 5.0306933582 && x < 5.0306933584);
    assert!(y > 7.5326602150 && y < 7.5326602152);
    assert!(z > -0.3347077 && z < -0.3347037);

    let (x, y, z) = vsop87e::saturn(2232395.0);

    assert!(x > 1.2645936161 && x < 1.2645936163);
    assert!(y > -10.0240954526 && y < -10.0240954524);
    assert!(z > 0.1345888 && z < 0.1345928);

    let (x, y, z) = vsop87e::saturn(2195870.0);

    assert!(x > -7.1627672768 && x < -7.1627672766);
    assert!(y > 5.7452916043 && y < 5.7452916045);
    assert!(z > 0.1724887 && z < 0.1724927);

    let (x, y, z) = vsop87e::saturn(2159345.0);

    assert!(x > 9.3505836059 && x < 9.3505836061);
    assert!(y > -2.1105535821 && y < -2.1105535819);
    assert!(z > -0.3230966 && z < -0.3230926);

    let (x, y, z) = vsop87e::saturn(2122820.0);

    assert!(x > -7.9370266648 && x < -7.9370266646);
    assert!(y > -5.8484785040 && y < -5.8484785038);
    assert!(z > 0.4164638 && z < 0.4164678);
}

#[test]
fn it_uranus() {
    let (x, y, z) = vsop87e::uranus(2451545.0);

    assert!(x > 14.4247519567 && x < 14.4247519569);
    assert!(y > -13.7371045088 && y < -13.7371045086);
    assert!(z > -0.2379381 && z < -0.2379341);

    let (x, y, z) = vsop87e::uranus(2415020.0);

    assert!(x > -6.4778956414 && x < -6.4778956412);
    assert!(y > -17.8463318323 && y < -17.8463318321);
    assert!(z > 0.0176878 && z < 0.0176918);

    let (x, y, z) = vsop87e::uranus(2378495.0);

    assert!(x > -18.2673444261 && x < -18.2673444259);
    assert!(y > 0.9819574798 && y < 0.9819574800);
    assert!(z > 0.2419649 && z < 0.2419689);

    let (x, y, z) = vsop87e::uranus(2341970.0);

    assert!(x > -4.2265696171 && x < -4.2265696169);
    assert!(y > 18.3208630948 && y < 18.3208630950);
    assert!(z > 0.1248825 && z < 0.1248865);

    let (x, y, z) = vsop87e::uranus(2305445.0);

    assert!(x > 16.1091090967 && x < 16.1091090969);
    assert!(y > 11.4867931079 && y < 11.4867931081);
    assert!(z > -0.1666680 && z < -0.1666640);

    let (x, y, z) = vsop87e::uranus(2268920.0);

    assert!(x > 17.7611606375 && x < 17.7611606377);
    assert!(y > -9.2405618854 && y < -9.2405618852);
    assert!(z > -0.2678679 && z < -0.2678639);

    let (x, y, z) = vsop87e::uranus(2232395.0);

    assert!(x > -0.7823849125 && x < -0.7823849123);
    assert!(y > -19.2505578585 && y < -19.2505578583);
    assert!(z > -0.0637589 && z < -0.0637549);

    let (x, y, z) = vsop87e::uranus(2195870.0);

    assert!(x > -17.6538791198 && x < -17.6538791196);
    assert!(y > -5.1666300881 && y < -5.1666300879);
    assert!(z > 0.2124594 && z < 0.2124634);

    let (x, y, z) = vsop87e::uranus(2159345.0);

    assert!(x > -9.8292937631 && x < -9.8292937629);
    assert!(y > 15.7752257094 && y < 15.7752257096);
    assert!(z > 0.1915092 && z < 0.1915132);

    let (x, y, z) = vsop87e::uranus(2122820.0);

    assert!(x > 11.8571754272 && x < 11.8571754274);
    assert!(y > 15.5546452201 && y < 15.5546452203);
    assert!(z > -0.0951153 && z < -0.0951113);
}

#[test]
fn it_neptune() {
    let (x, y, z) = vsop87e::neptune(2451545.0);

    assert!(x > 16.8049701268 && x < 16.8049701270);
    assert!(y > -24.9944513570 && y < -24.9944513568);
    assert!(z > 0.1274231 && z < 0.1274271);

    let (x, y, z) = vsop87e::neptune(2415020.0);

    assert!(x > 1.5196434116 && x < 1.5196434118);
    assert!(y > 29.8318114918 && y < 29.8318114920);
    assert!(z > -0.6492457 && z < -0.6492417);

    let (x, y, z) = vsop87e::neptune(2378495.0);

    assert!(x > -20.3104052893 && x < -20.3104052891);
    assert!(y > -22.4966336301 && y < -22.4966336299);
    assert!(z > 0.9308456 && z < 0.9308496);

    let (x, y, z) = vsop87e::neptune(2341970.0);

    assert!(x > 29.4971507219 && x < 29.4971507221);
    assert!(y > 4.6036063556 && y < 4.6036063558);
    assert!(z > -0.7739275 && z < -0.7739235);

    let (x, y, z) = vsop87e::neptune(2305445.0);

    assert!(x > -26.5753161425 && x < -26.5753161423);
    assert!(y > 14.1902813793 && y < 14.1902813795);
    assert!(z > 0.3194782 && z < 0.3194822);

    let (x, y, z) = vsop87e::neptune(2268920.0);

    assert!(x > 11.1089045477 && x < 11.1089045479);
    assert!(y > -28.0532332602 && y < -28.0532332600);
    assert!(z > 0.3218536 && z < 0.3218576);

    let (x, y, z) = vsop87e::neptune(2232395.0);

    assert!(x > 8.0258639137 && x < 8.0258639139);
    assert!(y > 28.7261896413 && y < 28.7261896415);
    assert!(z > -0.7760224 && z < -0.7760184);

    let (x, y, z) = vsop87e::neptune(2195870.0);

    assert!(x > -24.6233894508 && x < -24.6233894506);
    assert!(y > -17.6544160440 && y < -17.6544160438);
    assert!(z > 0.9297169 && z < 0.9297209);

    let (x, y, z) = vsop87e::neptune(2159345.0);

    assert!(x > 29.8297729718 && x < 29.8297729720);
    assert!(y > -2.0298541973 && y < -2.0298541971);
    assert!(z > -0.6440972 && z < -0.6440932);

    let (x, y, z) = vsop87e::neptune(2122820.0);

    assert!(x > -22.7959876639 && x < -22.7959876637);
    assert!(y > 19.5945850297 && y < 19.5945850299);
    assert!(z > 0.1205410 && z < 0.1205450);
}
