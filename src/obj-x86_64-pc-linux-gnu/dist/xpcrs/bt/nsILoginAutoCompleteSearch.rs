//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginAutoCompleteSearch.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginAutoCompleteSearch",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void startSearch (in AString aSearchString, in nsIAutoCompleteResult aPreviousResult, in HTMLInputElement aElement, in nsIFormAutoCompleteObserver aListener); */
                    Method {
                        name: "StartSearch",
                        params: &[Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }, Param { name: "aPreviousResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aListener", ty: "*const nsIFormAutoCompleteObserver" }],
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

        ]; D}

