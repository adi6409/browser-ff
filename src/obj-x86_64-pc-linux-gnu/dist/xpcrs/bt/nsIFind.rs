//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFind",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute boolean findBackwards; */
                    Method {
                        name: "GetFindBackwards",
                        params: &[Param { name: "aFindBackwards", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFindBackwards",
                        params: &[Param { name: "aFindBackwards", ty: "bool" }],
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

                    /* Range Find (in AString aPatText, in Range aSearchRange, in Range aStartPoint, in Range aEndPoint); */
                    Method {
                        name: "Find",
                        params: &[Param { name: "aPatText", ty: "*const ::nsstring::nsAString" }, Param { name: "aSearchRange", ty: "*const libc::c_void" }, Param { name: "aStartPoint", ty: "*const libc::c_void" }, Param { name: "aEndPoint", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

