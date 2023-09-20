foreign_resource(ex, [c1, c2, c11, c21, c3, c4, c5, c6]).
foreign(c1, c, c1(+integer, [-integer])).
foreign(c2, c, c2(-integer)).
foreign(c11, c, c11(+atom, [-atom])).
foreign(c21, c, c21(+atom, -atom)).
foreign(c3, c, c3(+float, [-float])).
foreign(c4, c, c4(-float)).
foreign(c5, c, c5(+string,[-string])).
foreign(c6, c, c6(-string)).
:- load_foreign_resource(ex).
