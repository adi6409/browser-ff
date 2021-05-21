//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/spellchecker/nsIInlineSpellChecker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInlineSpellChecker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIEditorSpellCheck spellChecker; */
                    Method {
                        name: "GetSpellChecker",
                        params: &[Param { name: "aSpellChecker", ty: "*mut*const nsIEditorSpellCheck" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in nsIEditor aEditor); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aEditor", ty: "*const nsIEditor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cleanup (in boolean aDestroyingFrames); */
                    Method {
                        name: "Cleanup",
                        params: &[Param { name: "aDestroyingFrames", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean enableRealTimeSpell; */
                    Method {
                        name: "GetEnableRealTimeSpell",
                        params: &[Param { name: "aEnableRealTimeSpell", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEnableRealTimeSpell",
                        params: &[Param { name: "aEnableRealTimeSpell", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void spellCheckRange (in Range aSelection); */
                    Method {
                        name: "SpellCheckRange",
                        params: &[Param { name: "aSelection", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Range getMisspelledWord (in Node aNode, in long aOffset); */
                    Method {
                        name: "GetMisspelledWord",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "aOffset", ty: "i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void replaceWord (in Node aNode, in long aOffset, in AString aNewword); */
                    Method {
                        name: "ReplaceWord",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "aOffset", ty: "i32" }, Param { name: "aNewword", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addWordToDictionary (in AString aWord); */
                    Method {
                        name: "AddWordToDictionary",
                        params: &[Param { name: "aWord", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeWordFromDictionary (in AString aWord); */
                    Method {
                        name: "RemoveWordFromDictionary",
                        params: &[Param { name: "aWord", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void ignoreWord (in AString aWord); */
                    Method {
                        name: "IgnoreWord",
                        params: &[Param { name: "aWord", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void ignoreWords (in Array<AString> aWordsToIgnore); */
                    Method {
                        name: "IgnoreWords",
                        params: &[Param { name: "aWordsToIgnore", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateCurrentDictionary (); */
                    Method {
                        name: "UpdateCurrentDictionary",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean spellCheckPending; */
                    Method {
                        name: "GetSpellCheckPending",
                        params: &[Param { name: "aSpellCheckPending", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

