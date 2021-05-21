//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormAutoComplete.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormAutoComplete",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in HTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener, [optional] in nsIPropertyBag2 options); */
                    Method {
                        name: "AutoCompleteSearchAsync",
                        params: &[Param { name: "aInputName", ty: "*const ::nsstring::nsAString" }, Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }, Param { name: "aField", ty: "*const libc::c_void" }, Param { name: "aPreviousResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "aDatalistResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "aListener", ty: "*const nsIFormAutoCompleteObserver" }, Param { name: "options", ty: "*const nsIPropertyBag2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopAutoCompleteSearch (); */
                    Method {
                        name: "StopAutoCompleteSearch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFormAutoCompleteObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void onSearchCompletion (in nsIAutoCompleteResult result); */
                    Method {
                        name: "OnSearchCompletion",
                        params: &[Param { name: "result", ty: "*const nsIAutoCompleteResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

