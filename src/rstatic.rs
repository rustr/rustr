//! C static varibles for R
//!
//!
//!



use rdll::*;

#[inline]
pub fn r_nan() -> ::std::os::raw::c_double {
    unsafe { R_NaN }
}

macro_rules! rconst{
	($( ($funname:ident;$rettype:ty ;$varname:ident)),*)=>(
	
	$(
	pub fn $funname()->$rettype{ unsafe{$varname}}
	)*
	)
}

rconst!( (r_posinf; ::std::os::raw::c_double; R_PosInf),
		 (r_neginf; ::std::os::raw::c_double; R_NegInf),
		 (r_nareal; ::std::os::raw::c_double; R_NaReal),
		 (r_naint; ::std::os::raw::c_int; R_NaInt ),
		 (r_globalenv; SEXP;R_GlobalEnv ),
		 (r_emptyenv; SEXP;R_EmptyEnv ),
		 (r_baseenv; SEXP; R_BaseEnv),
		 (r_basenamespace; SEXP; R_BaseNamespace),
		 (r_namespaceregistry; SEXP; R_NamespaceRegistry),
		 (r_srcref; SEXP; R_Srcref),
		 (rnull; SEXP; R_NilValue),
		 (r_unboundvalue; SEXP; R_UnboundValue),
		 (r_missingarg; SEXP; R_MissingArg),
		 (r_basesymbol; SEXP; R_baseSymbol),
		 (r_bbasesymbol; SEXP; R_BaseSymbol),
		 (r_bracesymbol; SEXP; R_BraceSymbol),
		 (r_bracket2symbol; SEXP; R_Bracket2Symbol),
		 (r_bracketsymbol; SEXP; R_BracketSymbol),
		 (r_classsymbol; SEXP; R_ClassSymbol),
		 (r_devicesymbol; SEXP; R_DeviceSymbol),
		 (r_dimnamessymbol; SEXP; R_DimNamesSymbol),
		 (r_dimsymbol; SEXP; R_DimSymbol),
		 (r_dollarsymbol; SEXP; R_DollarSymbol),
		 (r_dotsymbol; SEXP; R_DotsSymbol),
		 (r_doublecolonsymbol; SEXP; R_DoubleColonSymbol),
		 (r_dropsymbol; SEXP; R_DropSymbol),
		 (r_lastvaluesymbol; SEXP; R_LastvalueSymbol),
		 (r_levelsymbol; SEXP; R_LevelsSymbol),
		 (r_modesymbol; SEXP; R_ModeSymbol),
		 (r_narmsymbol; SEXP; R_NaRmSymbol),
		 (r_namesymbol; SEXP; R_NameSymbol),
		 (r_namessymbol; SEXP; R_NamesSymbol),
		 (r_namespaceenvsymbol; SEXP; R_NamespaceEnvSymbol),
		 (r_packagesymbol; SEXP; R_PackageSymbol),
		 (r_previoussymbol; SEXP; R_PreviousSymbol),
		 (r_quotesymbol; SEXP; R_QuoteSymbol),
		 (r_rownamessymbol; SEXP; R_RowNamesSymbol),
		 (r_seedssymbol; SEXP; R_SeedsSymbol),
		 (r_sortlistsymbol; SEXP; R_SortListSymbol),
		 (r_sourcesymbol; SEXP; R_SourceSymbol),
		 (r_specsymbol; SEXP; R_SpecSymbol),
		 (r_triplecolonsymbol; SEXP; R_TripleColonSymbol),
		 (r_tspsymbol; SEXP; R_TspSymbol),
		 (r_dot_defined; SEXP; R_dot_defined),
		 (r_dot_method; SEXP; R_dot_Method),
		 (r_dot_packagename; SEXP; R_dot_packageName),
		 (r_dot_target; SEXP; R_dot_target),
// (r_dot_generic; SEXP; R_dot_Generic),
		 (r_blankstring; SEXP; R_BlankString),
		 (r_blankscalarstring; SEXP; R_BlankScalarString),
		 (r_nastring; SEXP; R_NaString),
// (r_dirtyimage;::std::os::raw::c_int; R_DirtyImage  ),
// (r_tempdir; *mut ::std::os::raw::c_char;R_TempDir  ),
		 (r_interrupts_suspended; Rboolean;R_interrupts_suspended),
		 (r_interrupts_pending;  ::std::os::raw::c_int;R_interrupts_pending),
		 (r_mbslocale;  Rboolean;mbcslocale),
		 (r_num_math_threads; ::std::os::raw::c_int;R_num_math_threads ),
		 (r_max_num_math_threads; ::std::os::raw::c_int; R_max_num_math_threads)
		  );


//    pub static mut R_num_math_threads: ::std::os::raw::c_int;
//    pub static mut R_max_num_math_threads: ::std::os::raw::c_int;
