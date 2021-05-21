//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFindService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFindService",
            base: Some("nsISupports"),
            methods: Ok(&[
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

                    /* attribute AString replaceString; */
                    Method {
                        name: "GetReplaceString",
                        params: &[Param { name: "aReplaceString", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetReplaceString",
                        params: &[Param { name: "aReplaceString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

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

                    /* attribute boolean wrapFind; */
                    Method {
                        name: "GetWrapFind",
                        params: &[Param { name: "aWrapFind", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetWrapFind",
                        params: &[Param { name: "aWrapFind", ty: "bool" }],
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

                    /* attribute boolean matchCase; */
                    Method {
                        name: "GetMatchCase",
                        params: &[Param { name: "aMatchCase", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMatchCase",
                        params: &[Param { name: "aMatchCase", ty: "bool" }],
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

                    ]),
        },

        ]; D}

