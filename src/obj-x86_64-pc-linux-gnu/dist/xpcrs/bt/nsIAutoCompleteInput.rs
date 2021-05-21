//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteInput.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteInput",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Element popupElement; */
                    Method {
                        name: "GetPopupElement",
                        params: &[Param { name: "aPopupElement", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAutoCompletePopup popup; */
                    Method {
                        name: "GetPopup",
                        params: &[Param { name: "aPopup", ty: "*mut*const nsIAutoCompletePopup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAutoCompleteController controller; */
                    Method {
                        name: "GetController",
                        params: &[Param { name: "aController", ty: "*mut *const nsIAutoCompleteController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] attribute boolean popupOpen; */
                    Method {
                        name: "GetPopupOpen",
                        params: &[Param { name: "aPopupOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPopupOpen",
                        params: &[Param { name: "aPopupOpen", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean disableAutoComplete; */
                    Method {
                        name: "GetDisableAutoComplete",
                        params: &[Param { name: "aDisableAutoComplete", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDisableAutoComplete",
                        params: &[Param { name: "aDisableAutoComplete", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean completeDefaultIndex; */
                    Method {
                        name: "GetCompleteDefaultIndex",
                        params: &[Param { name: "aCompleteDefaultIndex", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCompleteDefaultIndex",
                        params: &[Param { name: "aCompleteDefaultIndex", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean completeSelectedIndex; */
                    Method {
                        name: "GetCompleteSelectedIndex",
                        params: &[Param { name: "aCompleteSelectedIndex", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCompleteSelectedIndex",
                        params: &[Param { name: "aCompleteSelectedIndex", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean forceComplete; */
                    Method {
                        name: "GetForceComplete",
                        params: &[Param { name: "aForceComplete", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetForceComplete",
                        params: &[Param { name: "aForceComplete", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long minResultsForPopup; */
                    Method {
                        name: "GetMinResultsForPopup",
                        params: &[Param { name: "aMinResultsForPopup", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMinResultsForPopup",
                        params: &[Param { name: "aMinResultsForPopup", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long maxRows; */
                    Method {
                        name: "GetMaxRows",
                        params: &[Param { name: "aMaxRows", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxRows",
                        params: &[Param { name: "aMaxRows", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long timeout; */
                    Method {
                        name: "GetTimeout",
                        params: &[Param { name: "aTimeout", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTimeout",
                        params: &[Param { name: "aTimeout", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString searchParam; */
                    Method {
                        name: "GetSearchParam",
                        params: &[Param { name: "aSearchParam", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchParam",
                        params: &[Param { name: "aSearchParam", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long searchCount; */
                    Method {
                        name: "GetSearchCount",
                        params: &[Param { name: "aSearchCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getSearchAt (in unsigned long index); */
                    Method {
                        name: "GetSearchAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString textValue; */
                    Method {
                        name: "GetTextValue",
                        params: &[Param { name: "aTextValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTextValue",
                        params: &[Param { name: "aTextValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
                    Method {
                        name: "SetTextValueWithReason",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aReason", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long selectionStart; */
                    Method {
                        name: "GetSelectionStart",
                        params: &[Param { name: "aSelectionStart", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long selectionEnd; */
                    Method {
                        name: "GetSelectionEnd",
                        params: &[Param { name: "aSelectionEnd", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectTextRange (in long startIndex, in long endIndex); */
                    Method {
                        name: "SelectTextRange",
                        params: &[Param { name: "startIndex", ty: "i32" }, Param { name: "endIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSearchBegin (); */
                    Method {
                        name: "OnSearchBegin",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSearchComplete (); */
                    Method {
                        name: "OnSearchComplete",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean onTextEntered ([optional] in Event aEvent, [optional] in boolean itemWasSelected); */
                    Method {
                        name: "OnTextEntered",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "itemWasSelected", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean onTextReverted (); */
                    Method {
                        name: "OnTextReverted",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean consumeRollupEvent; */
                    Method {
                        name: "GetConsumeRollupEvent",
                        params: &[Param { name: "aConsumeRollupEvent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean inPrivateContext; */
                    Method {
                        name: "GetInPrivateContext",
                        params: &[Param { name: "aInPrivateContext", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean noRollupOnCaretMove; */
                    Method {
                        name: "GetNoRollupOnCaretMove",
                        params: &[Param { name: "aNoRollupOnCaretMove", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean noRollupOnEmptySearch; */
                    Method {
                        name: "GetNoRollupOnEmptySearch",
                        params: &[Param { name: "aNoRollupOnEmptySearch", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long userContextId; */
                    Method {
                        name: "GetUserContextId",
                        params: &[Param { name: "aUserContextId", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

