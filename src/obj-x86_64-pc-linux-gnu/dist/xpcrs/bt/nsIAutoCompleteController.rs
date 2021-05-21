//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [setter_can_run_script] attribute nsIAutoCompleteInput input; */
                    Method {
                        name: "GetInput",
                        params: &[Param { name: "aInput", ty: "*mut*const nsIAutoCompleteInput" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetInput",
                        params: &[Param { name: "aInput", ty: "*const nsIAutoCompleteInput" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short searchStatus; */
                    Method {
                        name: "GetSearchStatus",
                        params: &[Param { name: "aSearchStatus", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long matchCount; */
                    Method {
                        name: "GetMatchCount",
                        params: &[Param { name: "aMatchCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void startSearch (in AString searchString); */
                    Method {
                        name: "StartSearch",
                        params: &[Param { name: "searchString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void stopSearch (); */
                    Method {
                        name: "StopSearch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] boolean handleText (); */
                    Method {
                        name: "HandleText",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] boolean handleEnter (in boolean aIsPopupSelection, [optional] in Event aEvent); */
                    Method {
                        name: "HandleEnter",
                        params: &[Param { name: "aIsPopupSelection", ty: "bool" }, Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] boolean handleEscape (); */
                    Method {
                        name: "HandleEscape",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void handleStartComposition (); */
                    Method {
                        name: "HandleStartComposition",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleEndComposition (); */
                    Method {
                        name: "HandleEndComposition",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void handleTab (); */
                    Method {
                        name: "HandleTab",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] boolean handleKeyNavigation (in unsigned long key); */
                    Method {
                        name: "HandleKeyNavigation",
                        params: &[Param { name: "key", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] boolean handleDelete (); */
                    Method {
                        name: "HandleDelete",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getValueAt (in long index); */
                    Method {
                        name: "GetValueAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getLabelAt (in long index); */
                    Method {
                        name: "GetLabelAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getCommentAt (in long index); */
                    Method {
                        name: "GetCommentAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStyleAt (in long index); */
                    Method {
                        name: "GetStyleAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getImageAt (in long index); */
                    Method {
                        name: "GetImageAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getFinalCompleteValueAt (in long index); */
                    Method {
                        name: "GetFinalCompleteValueAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString searchString; */
                    Method {
                        name: "GetSearchString",
                        params: &[Param { name: "aSearchString", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchString",
                        params: &[Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setInitiallySelectedIndex (in long index); */
                    Method {
                        name: "SetInitiallySelectedIndex",
                        params: &[Param { name: "index", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void resetInternalState (); */
                    Method {
                        name: "ResetInternalState",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

