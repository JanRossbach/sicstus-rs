/*
* Exposes KISSAT API to SICStus Prolog
*
* Based on SWI Prolog Minisat Integration by
* Michael Codish, Vitaly Lagoon, Peter J. Stuckey
*/

%% :- module(satsolver, [init_satsolver/0, bt_sat/1,
%%                      solve_predicate_with_satsolver_free/2,
%%                      solve_predicate_with_satsolver_in_state/3,
%%                      solve_predicate_with_satsolver/2]).

%% :- use_module('../../src/module_information.pl').
%% :- module_info(group,experimental).
%% :- module_info(description,'This is the interface between ProB and the Glucose SAT solver').

%% :- use_module(probsrc(tools),[start_ms_timer/1, stop_ms_timer_with_msg/2, stop_ms_timer_with_debug_msg/2]).
%% :- use_module(probsrc(debug),[debug_format/3]).
%% :- use_module(b_to_cnf).

%% :- use_module(library(lists), [maplist/2,maplist/3,exclude/3]).

foreign_resource('satsolver', [new_solver,solve,delete_solver,
                               add_clause,assign_model,get_model,toDimacs]).

foreign(new_solver, c, new_solver([-integer])).
foreign(solve, c, solve(+integer)).
foreign(delete_solver, c, delete_solver(+integer)).
foreign(add_clause, c, add_clause(+integer,+term)).
foreign(assign_model, c, assign_model(+integer,+term)).
foreign(get_model, c, get_model(+integer,+term)).
foreign(toDimacs, c, toDimacs(+integer,+string)).

:- load_foreign_resource(satsolver).

%% :- dynamic is_initialised/0.

%% init_satsolver :- is_initialised,!.
%% init_satsolver :-
%%     catch(load_foreign_resource(library(satsolver)),E,
%%        (format(user_error,'*** LOADING satsolver library failed!~n ~w~n',[E]),fail)),
%%     assertz(is_initialised).


%% :- use_module(probsrc(bsyntaxtree), [find_typed_identifier_uses/2, def_get_texpr_id/2]).
%% :- use_module(probsrc(solver_interface), [set_up_typed_localstate_for_pred/4]).
%% :- use_module(probsrc(b_compiler),[b_optimize/6]).

%% % call to solve a predicate with sat solver assuming all variables are free:
%% solve_predicate_with_satsolver_free(BFormula,Result) :-
%%     find_typed_identifier_uses(BFormula,UsedIdentifiers),
%%     set_up_typed_localstate_for_pred(UsedIdentifiers,BFormula,TypedVals,LocalState),
%%     b_optimize(BFormula,[],LocalState,[],NewTyped,no_wf_available),
%%     solve_predicate_with_satsolver(NewTyped,TypedVals,LocalState,Result).


%% % call to solve a predicate with sat solver with a provided state with variable/constant values:
%% solve_predicate_with_satsolver_in_state(BFormula,State,Result) :-
%%     find_typed_identifier_uses(BFormula,UsedIdentifiers),
%%     exclude(is_already_declared_in_state(State),UsedIdentifiers,FilteredIdentifiers),
%%     set_up_typed_localstate_for_pred(FilteredIdentifiers,BFormula,TypedVals,LocalState),
%%     b_optimize(BFormula,[],LocalState,State,NewTyped,no_wf_available),
%%     append(LocalState,State,State2),
%%     solve_predicate_with_satsolver(NewTyped,TypedVals,State2,Result).
%% is_already_declared_in_state(State,TID) :- def_get_texpr_id(TID,ID), memberchk(bind(ID,_),State).

%% solve_predicate_with_satsolver(BFormula,State) :-
%%    solve_predicate_with_satsolver(BFormula,[],State,Result),
%%    Result = solution(_).

%% solve_predicate_with_satsolver(BFormula,TypedVals,State,Result) :-
%%   %write(' :sat '),nl, translate:nested_print_bexpr(BFormula),nl,
%%   init_satsolver,!,
%%   if(solve_pred2(BFormula,TypedVals,State,Result),true,Result=error).
%% solve_predicate_with_satsolver(_,_,_,error).

%% :- use_module(probsrc(b_enumerate), [b_tighter_enumerate_all_values/2]).
%% :- use_module(probsrc(kernel_waitflags),
%%               [init_wait_flags_with_call_stack/2,ground_wait_flags/1,
%%                ground_det_wait_flag/1]).

%% solve_pred2(BFormula,TypedVals,State,Result) :-
%%   init_wait_flags_with_call_stack(WF,[prob_command_context(sat_solving,unknown)]),
%%   if(solve_pred3(BFormula,State,Result,WF),
%%      (b_tighter_enumerate_all_values(TypedVals,WF),
%%       ground_wait_flags(WF)),
%%      Result=contradiction_found).

%% solve_pred3(BFormula,State,Result,WF) :-
%%   start_ms_timer(T1),
%%   % Note: b_to_cnf will also look up all identifiers
%%   (b_to_cnf_wf(BFormula,State,CNF,WF)
%%    -> stop_ms_timer_with_msg(T1,b_to_cnf),
%%       ground_det_wait_flag(WF),
%%       start_ms_timer(T2),
%%       %portray_cnf(CNF),
%%       solve_cnf(T2,CNF,State,Result)
%%    ; Result=error
%%    ).

%% solve_cnf(T2,CNF,State,Result) :-
%%   bt_sat(CNF),
%%   stop_ms_timer_with_msg(T2,'Formula is SATisfiable'),
%%   Result = solution(State).
%% solve_cnf(T2,_,_,_Result) :-
%%   stop_ms_timer_with_msg(T2,'Formula is UNSATisfiable'),
%%   fail. % fail to avoid pending co-routines of b_to_cnf

%% :- public portray_cnf/1.
%% portray_cnf(CNF) :- write('CNF:'),nl,maplist(portray_cl,CNF).
%% portray_cl(Clause) :- format('  ~w~n',[Clause]).

%% % --------------

%% % Not used:
%% %sat(CNF):-
%% %    sat(CNF,Solved),!,
%% %    Solved=1.
%% %
%% %sat([],1):-!.
%% %sat([[]],0):-!.
%% %sat(F,Solved):-
%% %    new_solver(SolverID),
%% %    setup_solver(SolverID),
%% %    (addCnf2Solver(SolverID,F,FVars,_)
%% %    ->(solve(SolverID)
%% %        -> assign_model(SolverID,[1|FVars]), % Note: assign model can fail when ProB triggers co-routines
%% %           Solved = 1
%% %        ;  Solved = 0)
%% %    ; Solved=0),
%% %    delete_solver(SolverID),!.

%% % backtracking solving:
%% bt_sat(CNF) :-
%%     new_solver(SolverID),
%%     bt_sat_aux(SolverID,CNF).

%% bt_sat_aux(SolverID,F) :-
%%     setup_solver(SolverID),
%%     addCnf2Solver(SolverID,F,FVars,NrVars),
%%     %toDimacs(SolverID,'satsolver_dimacs.cnf'), % comment in to export CNF to Dimacs format
%%     debug_format(19,'Solving for ~w sat variables~n',[NrVars]),
%%     solve(SolverID),
%%     debug_format(19,'Get Model~n',[]),
%%     get_model(SolverID,[_|Model]),
%%     debug_format(19,'Model = ~w~n',[Model]),
%%     bt_sat(SolverID,FVars,Model).
%% bt_sat_aux(SolverID,_) :-
%%     delete_solver(SolverID), fail.

%% bt_sat(SolverID,FVars,_Model) :-
%%     debug_format(19,'Assigning Model~n',[]),
%%     assign_model(SolverID,[1|FVars]). % Note: assign model can fail when ProB triggers co-routines
%% bt_sat(SolverID,FVars,Model) :-
%%     debug_format(19,'Adding negated model to find another solution~n',[]),
%%     add_negated_model(SolverID,Model),
%%     solve(SolverID),
%%     get_model(SolverID,[_|NewModel]),
%%     bt_sat(SolverID,FVars,NewModel).

%% add_negated_model(SolverID,Model):-
%%     maplist(neg,Model,NoAsgn),
%%     add_clause(SolverID,NoAsgn).

%% neg(V,VN) :- VN is -V.

%% % get a fixed maximum number of models in one go:
%% %multi_sat([],_,1):-!.
%% %multi_sat([[]],_,0):-!.
%% %multi_sat(F,MaxSols,SolCount):-
%% %    new_solver(SolverID),
%% %    setup_solver(SolverID),
%% %    (addCnf2Solver(SolverID,F,FVars)
%% %    -> satMultiModels(SolverID,MaxSols,Models),
%% %       delete_solver(SolverID),!,
%% %       length(Models,SolCount),
%% %       (SolCount == 0 ; assignMultiSols(Models,FVars))
%% %    ;  delete_solver(SolverID),!,SolCount=0),!.
%% %satMultiModels(SolverID,MaxSols,Models):-
%% %    MaxSols > 0,!,
%% %    (solve(SolverID)
%% %    -> get_model(SolverID,[_|Model]),
%% %       Models=[Model|MoreModels],
%% %       MaxSols1 is MaxSols - 1,
%% %       (MaxSols1 > 0,
%% %        add_negated_model(SolverID,Model)
%% %        -> satMultiModels(SolverID,MaxSols1,MoreModels)
%% %         ; MoreModels=[])
%% %    ; Models=[]).
%% %satMultiModels(_,_,[]):-!.
%% %
%% %assignMultiSols(Models,FVars):-!,
%% %    length(FVars,VarLen),
%% %    length(SoFar,VarLen),
%% %    maplist(=([]),SoFar),!,
%% %    assignMultiSols(Models,SoFar,FVars).
%% %
%% %assignMultiSols([],SoFar,SoFar):-!.
%% %assignMultiSols([M|Models],SoFar,Vars):-!,
%% %    addModel2Vars(M,SoFar,NVars),!,
%% %    assignMultiSols(Models,NVars,Vars).
%% %
%% %addModel2Vars([],[],[]).
%% %addModel2Vars([M|Ms],[V|Vs],[[NV|V]|NVs]):-
%% %    (M>0 -> NV=1 ; NV= -1),
%% %    addModel2Vars(Ms,Vs,NVs).

%% addCnf2Solver(SolverID,Cnf,FVars,NrVars):-
%%     term_variables(Cnf,FVars),!,
%%     length(FVars,NrVars), length(Cnf,ClNr),
%%     format('Clauses: ~w, SAT Variables: ~w, Solver ID: ~w ~n',[ClNr,NrVars,SolverID]),
%%     \+ \+ (bind2index(FVars,3,FN),
%%            %portray_cnf(Cnf),
%%            add_cnf_clauses(Cnf,SolverID),
%%            neg(FN,FNeg),
%%            add_cnf_clauses([[FN,FNeg]],SolverID)).

%% add_cnf_clauses([Cl|Cls],SolverID):-!,
%%     maplist(to_minisat_cl,Cl,MiniSatCl),
%%     add_clause(SolverID,MiniSatCl),
%%     add_cnf_clauses(Cls,SolverID).
%% add_cnf_clauses([],_):-!.

%% setup_solver(SolverID) :-
%%     add_clause(SolverID,[1]),  % true literal
%%     add_clause(SolverID,[-2]). % false literal

%% to_minisat_cl(pred_true,1) :- !.
%% to_minisat_cl(pred_false,2) :- !.
%% to_minisat_cl(X,X).

%% % bind a list of variables to numbers (representing literals in .cnf)
%% bind2index([N|Ns],N,FN) :- N1 is N+1, bind2index(Ns,N1,FN).
%% bind2index([],N,FN):-!, FN is N - 1.
