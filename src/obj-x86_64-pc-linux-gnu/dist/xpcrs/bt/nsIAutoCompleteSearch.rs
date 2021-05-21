//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSearch.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteSearch",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener, [optional] in nsIPropertyBag2 options); */
                    Method {
                        name: "StartSearch",
                        params: &[Param { name: "searchString", ty: "*const ::nsstring::nsAString" }, Param { name: "searchParam", ty: "*const ::nsstring::nsAString" }, Param { name: "previousResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "listener", ty: "*const nsIAutoCompleteObserver" }, Param { name: "options", ty: "*const nsIPropertyBag2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopSearch (); */
                    Method {
                        name: "StopSearch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAutoCompleteObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
                    Method {
                        name: "OnSearchResult",
                        params: &[Param { name: "search", ty: "*const nsIAutoCompleteSearch" }, Param { name: "result", ty: "*const nsIAutoCompleteResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAutoCompleteSearchDescriptor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned short searchType; */
                    Method {
                        name: "GetSearchType",
                        params: &[Param { name: "aSearchType", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean clearingAutoFillSearchesAgain; */
                    Method {
                        name: "GetClearingAutoFillSearchesAgain",
                        params: &[Param { name: "aClearingAutoFillSearchesAgain", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

