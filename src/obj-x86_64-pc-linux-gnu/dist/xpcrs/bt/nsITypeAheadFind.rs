//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/typeaheadfind/nsITypeAheadFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITypeAheadFind",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in nsIDocShell aDocShell); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned short find (in AString aSearchString, in boolean aLinksOnly, in unsigned long aMode, in boolean aDontIterateFrames); */
                    Method {
                        name: "Find",
                        params: &[Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }, Param { name: "aLinksOnly", ty: "bool" }, Param { name: "aMode", ty: "u32" }, Param { name: "aDontIterateFrames", ty: "bool" }, Param { name: "_retval", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Range getFoundRange (); */
                    Method {
                        name: "GetFoundRange",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDocShell (in nsIDocShell aDocShell); */
                    Method {
                        name: "SetDocShell",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSelectionModeAndRepaint (in short toggle); */
                    Method {
                        name: "SetSelectionModeAndRepaint",
                        params: &[Param { name: "toggle", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void collapseSelection (); */
                    Method {
                        name: "CollapseSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isRangeVisible (in Range aRange, in boolean aMustBeInViewPort); */
                    Method {
                        name: "IsRangeVisible",
                        params: &[Param { name: "aRange", ty: "*const libc::c_void" }, Param { name: "aMustBeInViewPort", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isRangeRendered (in Range aRange); */
                    Method {
                        name: "IsRangeRendered",
                        params: &[Param { name: "aRange", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString searchString; */
                    Method {
                        name: "GetSearchString",
                        params: &[Param { name: "aSearchString", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean caseSensitive; */
                    Method {
                        name: "GetCaseSensitive",
                        params: &[Param { name: "aCaseSensitive", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCaseSensitive",
                        params: &[Param { name: "aCaseSensitive", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean matchDiacritics; */
                    Method {
                        name: "GetMatchDiacritics",
                        params: &[Param { name: "aMatchDiacritics", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMatchDiacritics",
                        params: &[Param { name: "aMatchDiacritics", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean entireWord; */
                    Method {
                        name: "GetEntireWord",
                        params: &[Param { name: "aEntireWord", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEntireWord",
                        params: &[Param { name: "aEntireWord", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element foundLink; */
                    Method {
                        name: "GetFoundLink",
                        params: &[Param { name: "aFoundLink", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element foundEditable; */
                    Method {
                        name: "GetFoundEditable",
                        params: &[Param { name: "aFoundEditable", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindow currentWindow; */
                    Method {
                        name: "GetCurrentWindow",
                        params: &[Param { name: "aCurrentWindow", ty: "*mut*const mozIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

