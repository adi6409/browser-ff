//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString searchString; */
                    Method {
                        name: "GetSearchString",
                        params: &[Param { name: "aSearchString", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short searchResult; */
                    Method {
                        name: "GetSearchResult",
                        params: &[Param { name: "aSearchResult", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long defaultIndex; */
                    Method {
                        name: "GetDefaultIndex",
                        params: &[Param { name: "aDefaultIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString errorDescription; */
                    Method {
                        name: "GetErrorDescription",
                        params: &[Param { name: "aErrorDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long matchCount; */
                    Method {
                        name: "GetMatchCount",
                        params: &[Param { name: "aMatchCount", ty: "*mut u32" }],
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

                    /* void removeValueAt (in long rowIndex); */
                    Method {
                        name: "RemoveValueAt",
                        params: &[Param { name: "rowIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

