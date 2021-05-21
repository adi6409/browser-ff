//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsITextInputProcessorCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITextInputProcessorNotification",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long offset; */
                    Method {
                        name: "GetOffset",
                        params: &[Param { name: "aOffset", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "GetText",
                        params: &[Param { name: "aText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean collapsed; */
                    Method {
                        name: "GetCollapsed",
                        params: &[Param { name: "aCollapsed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean reversed; */
                    Method {
                        name: "GetReversed",
                        params: &[Param { name: "aReversed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString writingMode; */
                    Method {
                        name: "GetWritingMode",
                        params: &[Param { name: "aWritingMode", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean causedByComposition; */
                    Method {
                        name: "GetCausedByComposition",
                        params: &[Param { name: "aCausedByComposition", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean causedBySelectionEvent; */
                    Method {
                        name: "GetCausedBySelectionEvent",
                        params: &[Param { name: "aCausedBySelectionEvent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean occurredDuringComposition; */
                    Method {
                        name: "GetOccurredDuringComposition",
                        params: &[Param { name: "aOccurredDuringComposition", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long removedLength; */
                    Method {
                        name: "GetRemovedLength",
                        params: &[Param { name: "aRemovedLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long addedLength; */
                    Method {
                        name: "GetAddedLength",
                        params: &[Param { name: "aAddedLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean causedOnlyByComposition; */
                    Method {
                        name: "GetCausedOnlyByComposition",
                        params: &[Param { name: "aCausedOnlyByComposition", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean includingChangesDuringComposition; */
                    Method {
                        name: "GetIncludingChangesDuringComposition",
                        params: &[Param { name: "aIncludingChangesDuringComposition", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean includingChangesWithoutComposition; */
                    Method {
                        name: "GetIncludingChangesWithoutComposition",
                        params: &[Param { name: "aIncludingChangesWithoutComposition", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITextInputProcessorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
                    Method {
                        name: "OnNotify",
                        params: &[Param { name: "aTextInputProcessor", ty: "*const nsITextInputProcessor" }, Param { name: "aNotification", ty: "*const nsITextInputProcessorNotification" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

