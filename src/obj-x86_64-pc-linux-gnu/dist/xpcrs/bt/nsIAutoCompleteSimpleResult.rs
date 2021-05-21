//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSimpleResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteSimpleResult",
            base: Some("nsIAutoCompleteResult"),
            methods: Ok(&[
                    /* void setSearchString (in AString aSearchString); */
                    Method {
                        name: "SetSearchString",
                        params: &[Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setErrorDescription (in AString aErrorDescription); */
                    Method {
                        name: "SetErrorDescription",
                        params: &[Param { name: "aErrorDescription", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDefaultIndex (in long aDefaultIndex); */
                    Method {
                        name: "SetDefaultIndex",
                        params: &[Param { name: "aDefaultIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSearchResult (in unsigned short aSearchResult); */
                    Method {
                        name: "SetSearchResult",
                        params: &[Param { name: "aSearchResult", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
                    Method {
                        name: "InsertMatchAt",
                        params: &[Param { name: "aIndex", ty: "i32" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aComment", ty: "*const ::nsstring::nsAString" }, Param { name: "aImage", ty: "*const ::nsstring::nsAString" }, Param { name: "aStyle", ty: "*const ::nsstring::nsAString" }, Param { name: "aFinalCompleteValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aLabel", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
                    Method {
                        name: "AppendMatch",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aComment", ty: "*const ::nsstring::nsAString" }, Param { name: "aImage", ty: "*const ::nsstring::nsAString" }, Param { name: "aStyle", ty: "*const ::nsstring::nsAString" }, Param { name: "aFinalCompleteValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aLabel", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeMatchAt (in long aIndex); */
                    Method {
                        name: "RemoveMatchAt",
                        params: &[Param { name: "aIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAutoCompleteSimpleResultListener getListener (); */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIAutoCompleteSimpleResultListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIAutoCompleteSimpleResultListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAutoCompleteSimpleResultListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue); */
                    Method {
                        name: "OnValueRemoved",
                        params: &[Param { name: "aResult", ty: "*const nsIAutoCompleteSimpleResult" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

