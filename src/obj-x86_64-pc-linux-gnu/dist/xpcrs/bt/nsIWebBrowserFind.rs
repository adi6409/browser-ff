//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIWebBrowserFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserFind",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean findNext (); */
                    Method {
                        name: "FindNext",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
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

                    /* attribute boolean searchFrames; */
                    Method {
                        name: "GetSearchFrames",
                        params: &[Param { name: "aSearchFrames", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchFrames",
                        params: &[Param { name: "aSearchFrames", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebBrowserFindInFrames",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute mozIDOMWindowProxy currentSearchFrame; */
                    Method {
                        name: "GetCurrentSearchFrame",
                        params: &[Param { name: "aCurrentSearchFrame", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentSearchFrame",
                        params: &[Param { name: "aCurrentSearchFrame", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute mozIDOMWindowProxy rootSearchFrame; */
                    Method {
                        name: "GetRootSearchFrame",
                        params: &[Param { name: "aRootSearchFrame", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRootSearchFrame",
                        params: &[Param { name: "aRootSearchFrame", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean searchSubframes; */
                    Method {
                        name: "GetSearchSubframes",
                        params: &[Param { name: "aSearchSubframes", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchSubframes",
                        params: &[Param { name: "aSearchSubframes", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean searchParentFrames; */
                    Method {
                        name: "GetSearchParentFrames",
                        params: &[Param { name: "aSearchParentFrames", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchParentFrames",
                        params: &[Param { name: "aSearchParentFrames", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

