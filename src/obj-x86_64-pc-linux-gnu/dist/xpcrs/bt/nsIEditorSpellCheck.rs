//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorSpellCheck.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditorSpellCheck",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean canSpellCheck (); */
                    Method {
                        name: "CanSpellCheck",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback); */
                    Method {
                        name: "InitSpellChecker",
                        params: &[Param { name: "editor", ty: "*const nsIEditor" }, Param { name: "enableSelectionChecking", ty: "bool" }, Param { name: "callback", ty: "*const nsIEditorSpellCheckCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] AString GetNextMisspelledWord (); */
                    Method {
                        name: "GetNextMisspelledWord",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString GetSuggestedWord (); */
                    Method {
                        name: "GetSuggestedWord",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean CheckCurrentWord (in AString suggestedWord); */
                    Method {
                        name: "CheckCurrentWord",
                        params: &[Param { name: "suggestedWord", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void ReplaceWord (in AString misspelledWord, in AString replaceWord, in boolean allOccurrences); */
                    Method {
                        name: "ReplaceWord",
                        params: &[Param { name: "misspelledWord", ty: "*const ::nsstring::nsAString" }, Param { name: "replaceWord", ty: "*const ::nsstring::nsAString" }, Param { name: "allOccurrences", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void IgnoreWordAllOccurrences (in AString word); */
                    Method {
                        name: "IgnoreWordAllOccurrences",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void GetPersonalDictionary (); */
                    Method {
                        name: "GetPersonalDictionary",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* AString GetPersonalDictionaryWord (); */
                    Method {
                        name: "GetPersonalDictionaryWord",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void AddWordToDictionary (in AString word); */
                    Method {
                        name: "AddWordToDictionary",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void RemoveWordFromDictionary (in AString word); */
                    Method {
                        name: "RemoveWordFromDictionary",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> GetDictionaryList (); */
                    Method {
                        name: "GetDictionaryList",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString GetCurrentDictionary (); */
                    Method {
                        name: "GetCurrentDictionary",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void SetCurrentDictionary (in ACString dictionary); */
                    Method {
                        name: "SetCurrentDictionary",
                        params: &[Param { name: "dictionary", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void UninitSpellChecker (); */
                    Method {
                        name: "UninitSpellChecker",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setFilterType (in unsigned long filterType); */
                    Method {
                        name: "SetFilterType",
                        params: &[Param { name: "filterType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback); */
                    Method {
                        name: "UpdateCurrentDictionary",
                        params: &[Param { name: "callback", ty: "*const nsIEditorSpellCheckCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIEditorSpellCheckCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void editorSpellCheckDone (); */
                    Method {
                        name: "EditorSpellCheckDone",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

