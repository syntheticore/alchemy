use crate::Solid;

pub fn export(
  _solid: &Solid,
  name: &str,
  description: &str,
  _geom_only: bool
) -> String {
  let file = format!(r###"
    ISO-10303-21;
    HEADER;
    FILE_DESCRIPTION(
    /* description */ ('{}'),
    /* implementation_level */ '2;1');

    FILE_NAME(
    /* name */ '{}.step',
    /* time_stamp */ '2021-01-16T22:35:27+01:00',
    /* author */ (''),
    /* organization */ (''),
    /* preprocessor_version */ 'Shapex',
    /* originating_system */ 'Shapex Geometric Modeling Kernel',
    /* authorisation */ '');

    FILE_SCHEMA (('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 3 1 1 }}'));
    ENDSEC;

    DATA;
    #10=MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(#13),#183);
    #11=SHAPE_REPRESENTATION_RELATIONSHIP('SRR','None',#190,#12);
    #12=ADVANCED_BREP_SHAPE_REPRESENTATION('',(#14),#182);
    #13=STYLED_ITEM('',(#199),#14);
    #14=MANIFOLD_SOLID_BREP('Box',#107);
    #15=FACE_OUTER_BOUND('',#21,.T.);
    #16=FACE_OUTER_BOUND('',#22,.T.);
    #17=FACE_OUTER_BOUND('',#23,.T.);
    #18=FACE_OUTER_BOUND('',#24,.T.);
    #19=FACE_OUTER_BOUND('',#25,.T.);
    #20=FACE_OUTER_BOUND('',#26,.T.);
    #21=EDGE_LOOP('',(#71,#72,#73,#74));
    #22=EDGE_LOOP('',(#75,#76,#77,#78));
    #23=EDGE_LOOP('',(#79,#80,#81,#82));
    #24=EDGE_LOOP('',(#83,#84,#85,#86));
    #25=EDGE_LOOP('',(#87,#88,#89,#90));
    #26=EDGE_LOOP('',(#91,#92,#93,#94));
    #27=LINE('',#157,#39);
    #28=LINE('',#159,#40);
    #29=LINE('',#161,#41);
    #30=LINE('',#162,#42);
    #31=LINE('',#165,#43);
    #32=LINE('',#167,#44);
    #33=LINE('',#168,#45);
    #34=LINE('',#171,#46);
    #35=LINE('',#173,#47);
    #36=LINE('',#174,#48);
    #37=LINE('',#176,#49);
    #38=LINE('',#177,#50);
    #39=VECTOR('',#131,10.);
    #40=VECTOR('',#132,10.);
    #41=VECTOR('',#133,10.);
    #42=VECTOR('',#134,10.);
    #43=VECTOR('',#137,10.);
    #44=VECTOR('',#138,10.);
    #45=VECTOR('',#139,10.);
    #46=VECTOR('',#142,10.);
    #47=VECTOR('',#143,10.);
    #48=VECTOR('',#144,10.);
    #49=VECTOR('',#147,10.);
    #50=VECTOR('',#148,10.);
    #51=VERTEX_POINT('',#155);
    #52=VERTEX_POINT('',#156);
    #53=VERTEX_POINT('',#158);
    #54=VERTEX_POINT('',#160);
    #55=VERTEX_POINT('',#164);
    #56=VERTEX_POINT('',#166);
    #57=VERTEX_POINT('',#170);
    #58=VERTEX_POINT('',#172);
    #59=EDGE_CURVE('',#51,#52,#27,.T.);
    #60=EDGE_CURVE('',#51,#53,#28,.T.);
    #61=EDGE_CURVE('',#54,#53,#29,.T.);
    #62=EDGE_CURVE('',#52,#54,#30,.T.);
    #63=EDGE_CURVE('',#52,#55,#31,.T.);
    #64=EDGE_CURVE('',#56,#54,#32,.T.);
    #65=EDGE_CURVE('',#55,#56,#33,.T.);
    #66=EDGE_CURVE('',#55,#57,#34,.T.);
    #67=EDGE_CURVE('',#58,#56,#35,.T.);
    #68=EDGE_CURVE('',#57,#58,#36,.T.);
    #69=EDGE_CURVE('',#57,#51,#37,.T.);
    #70=EDGE_CURVE('',#53,#58,#38,.T.);
    #71=ORIENTED_EDGE('',*,*,#59,.F.);
    #72=ORIENTED_EDGE('',*,*,#60,.T.);
    #73=ORIENTED_EDGE('',*,*,#61,.F.);
    #74=ORIENTED_EDGE('',*,*,#62,.F.);
    #75=ORIENTED_EDGE('',*,*,#63,.F.);
    #76=ORIENTED_EDGE('',*,*,#62,.T.);
    #77=ORIENTED_EDGE('',*,*,#64,.F.);
    #78=ORIENTED_EDGE('',*,*,#65,.F.);
    #79=ORIENTED_EDGE('',*,*,#66,.F.);
    #80=ORIENTED_EDGE('',*,*,#65,.T.);
    #81=ORIENTED_EDGE('',*,*,#67,.F.);
    #82=ORIENTED_EDGE('',*,*,#68,.F.);
    #83=ORIENTED_EDGE('',*,*,#69,.F.);
    #84=ORIENTED_EDGE('',*,*,#68,.T.);
    #85=ORIENTED_EDGE('',*,*,#70,.F.);
    #86=ORIENTED_EDGE('',*,*,#60,.F.);
    #87=ORIENTED_EDGE('',*,*,#70,.T.);
    #88=ORIENTED_EDGE('',*,*,#67,.T.);
    #89=ORIENTED_EDGE('',*,*,#64,.T.);
    #90=ORIENTED_EDGE('',*,*,#61,.T.);
    #91=ORIENTED_EDGE('',*,*,#69,.T.);
    #92=ORIENTED_EDGE('',*,*,#59,.T.);
    #93=ORIENTED_EDGE('',*,*,#63,.T.);
    #94=ORIENTED_EDGE('',*,*,#66,.T.);
    #95=PLANE('',#121);
    #96=PLANE('',#122);
    #97=PLANE('',#123);
    #98=PLANE('',#124);
    #99=PLANE('',#125);
    #100=PLANE('',#126);
    #101=ADVANCED_FACE('',(#15),#95,.T.);
    #102=ADVANCED_FACE('',(#16),#96,.T.);
    #103=ADVANCED_FACE('',(#17),#97,.T.);
    #104=ADVANCED_FACE('',(#18),#98,.T.);
    #105=ADVANCED_FACE('',(#19),#99,.T.);
    #106=ADVANCED_FACE('',(#20),#100,.F.);
    #107=CLOSED_SHELL('',(#101,#102,#103,#104,#105,#106));
    #108=DERIVED_UNIT_ELEMENT(#110,1.);
    #109=DERIVED_UNIT_ELEMENT(#185,-3.);
    #110=(
    MASS_UNIT()
    NAMED_UNIT(*)
    SI_UNIT(.KILO.,.GRAM.)
    );
    #111=DERIVED_UNIT((#108,#109));
    #112=MEASURE_REPRESENTATION_ITEM('density measure',
    POSITIVE_RATIO_MEASURE(7850.),#111);
    #113=PROPERTY_DEFINITION_REPRESENTATION(#118,#115);
    #114=PROPERTY_DEFINITION_REPRESENTATION(#119,#116);
    #115=REPRESENTATION('material name',(#117),#182);
    #116=REPRESENTATION('density',(#112),#182);
    #117=DESCRIPTIVE_REPRESENTATION_ITEM('Steel','Steel');
    #118=PROPERTY_DEFINITION('material property','material name',#192);
    #119=PROPERTY_DEFINITION('material property','density of part',#192);
    #120=AXIS2_PLACEMENT_3D('placement',#153,#127,#128);
    #121=AXIS2_PLACEMENT_3D('',#154,#129,#130);
    #122=AXIS2_PLACEMENT_3D('',#163,#135,#136);
    #123=AXIS2_PLACEMENT_3D('',#169,#140,#141);
    #124=AXIS2_PLACEMENT_3D('',#175,#145,#146);
    #125=AXIS2_PLACEMENT_3D('',#178,#149,#150);
    #126=AXIS2_PLACEMENT_3D('',#179,#151,#152);
    #127=DIRECTION('axis',(0.,0.,1.));
    #128=DIRECTION('refdir',(1.,0.,0.));
    #129=DIRECTION('center_axis',(1.,0.,0.));
    #130=DIRECTION('ref_axis',(0.,0.,-1.));
    #131=DIRECTION('',(0.,0.,1.));
    #132=DIRECTION('',(0.,1.,0.));
    #133=DIRECTION('',(0.,0.,-1.));
    #134=DIRECTION('',(0.,1.,0.));
    #135=DIRECTION('center_axis',(0.,0.,1.));
    #136=DIRECTION('ref_axis',(1.,0.,0.));
    #137=DIRECTION('',(-1.,0.,0.));
    #138=DIRECTION('',(1.,0.,0.));
    #139=DIRECTION('',(0.,1.,0.));
    #140=DIRECTION('center_axis',(-1.,0.,0.));
    #141=DIRECTION('ref_axis',(0.,0.,1.));
    #142=DIRECTION('',(0.,0.,-1.));
    #143=DIRECTION('',(0.,0.,1.));
    #144=DIRECTION('',(0.,1.,0.));
    #145=DIRECTION('center_axis',(-3.70074341541719E-17,0.,-1.));
    #146=DIRECTION('ref_axis',(-1.,0.,3.70074341541719E-17));
    #147=DIRECTION('',(1.,0.,-3.70074341541719E-17));
    #148=DIRECTION('',(-1.,0.,3.70074341541719E-17));
    #149=DIRECTION('center_axis',(0.,1.,0.));
    #150=DIRECTION('ref_axis',(0.,0.,1.));
    #151=DIRECTION('center_axis',(0.,1.,0.));
    #152=DIRECTION('ref_axis',(1.,0.,0.));
    #153=CARTESIAN_POINT('',(0.,0.,0.));
    #154=CARTESIAN_POINT('Origin',(30.,0.,20.));
    #155=CARTESIAN_POINT('',(30.,0.,-20.));
    #156=CARTESIAN_POINT('',(30.,0.,20.));
    #157=CARTESIAN_POINT('',(30.,0.,-20.));
    #158=CARTESIAN_POINT('',(30.,140.,-20.));
    #159=CARTESIAN_POINT('',(30.,0.,-20.));
    #160=CARTESIAN_POINT('',(30.,140.,20.));
    #161=CARTESIAN_POINT('',(30.,140.,-20.));
    #162=CARTESIAN_POINT('',(30.,0.,20.));
    #163=CARTESIAN_POINT('Origin',(-30.,0.,20.));
    #164=CARTESIAN_POINT('',(-30.,0.,20.));
    #165=CARTESIAN_POINT('',(30.,0.,20.));
    #166=CARTESIAN_POINT('',(-30.,140.,20.));
    #167=CARTESIAN_POINT('',(30.,140.,20.));
    #168=CARTESIAN_POINT('',(-30.,0.,20.));
    #169=CARTESIAN_POINT('Origin',(-30.,0.,-20.));
    #170=CARTESIAN_POINT('',(-30.,0.,-20.));
    #171=CARTESIAN_POINT('',(-30.,0.,20.));
    #172=CARTESIAN_POINT('',(-30.,140.,-20.));
    #173=CARTESIAN_POINT('',(-30.,140.,20.));
    #174=CARTESIAN_POINT('',(-30.,0.,-20.));
    #175=CARTESIAN_POINT('Origin',(30.,0.,-20.));
    #176=CARTESIAN_POINT('',(-30.,0.,-20.));
    #177=CARTESIAN_POINT('',(-30.,140.,-20.));
    #178=CARTESIAN_POINT('Origin',(0.,140.,0.));
    #179=CARTESIAN_POINT('Origin',(0.,0.,0.));
    #180=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.01),#184,
    'DISTANCE_ACCURACY_VALUE',
    'Maximum model space distance between geometric entities at asserted c
    onnectivities');
    #181=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.01),#184,
    'DISTANCE_ACCURACY_VALUE',
    'Maximum model space distance between geometric entities at asserted c
    onnectivities');
    #182=(
    GEOMETRIC_REPRESENTATION_CONTEXT(3)
    GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#180))
    GLOBAL_UNIT_ASSIGNED_CONTEXT((#184,#186,#187))
    REPRESENTATION_CONTEXT('','3D')
    );
    #183=(
    GEOMETRIC_REPRESENTATION_CONTEXT(3)
    GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#181))
    GLOBAL_UNIT_ASSIGNED_CONTEXT((#184,#186,#187))
    REPRESENTATION_CONTEXT('','3D')
    );
    #184=(
    LENGTH_UNIT()
    NAMED_UNIT(*)
    SI_UNIT(.MILLI.,.METRE.)
    );
    #185=(
    LENGTH_UNIT()
    NAMED_UNIT(*)
    SI_UNIT($,.METRE.)
    );
    #186=(
    NAMED_UNIT(*)
    PLANE_ANGLE_UNIT()
    SI_UNIT($,.RADIAN.)
    );
    #187=(
    NAMED_UNIT(*)
    SI_UNIT($,.STERADIAN.)
    SOLID_ANGLE_UNIT()
    );
    #188=SHAPE_DEFINITION_REPRESENTATION(#189,#190);
    #189=PRODUCT_DEFINITION_SHAPE('',$,#192);
    #190=SHAPE_REPRESENTATION('',(#120),#182);
    #191=PRODUCT_DEFINITION_CONTEXT('part definition',#196,'design');
    #192=PRODUCT_DEFINITION('(Unsaved)','(Unsaved)',#193,#191);
    #193=PRODUCT_DEFINITION_FORMATION('',$,#198);
    #194=PRODUCT_RELATED_PRODUCT_CATEGORY('(Unsaved)','(Unsaved)',(#198));
    #195=APPLICATION_PROTOCOL_DEFINITION('international standard',
    'automotive_design',2009,#196);
    #196=APPLICATION_CONTEXT(
    'Core Data for Automotive Mechanical Design Process');
    #197=PRODUCT_CONTEXT('part definition',#196,'mechanical');
    #198=PRODUCT('(Unsaved)','(Unsaved)',$,(#197));
    #199=PRESENTATION_STYLE_ASSIGNMENT((#200));
    #200=SURFACE_STYLE_USAGE(.BOTH.,#201);
    #201=SURFACE_SIDE_STYLE('',(#202));
    #202=SURFACE_STYLE_FILL_AREA(#203);
    #203=FILL_AREA_STYLE('Steel - Satin',(#204));
    #204=FILL_AREA_STYLE_COLOUR('Steel - Satin',#205);
    #205=COLOUR_RGB('Steel - Satin',0.627450980392157,0.627450980392157,0.627450980392157);
    ENDSEC;
    END-ISO-10303-21;
  "###, description, name);
  file
}