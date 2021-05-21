//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozIPersonalDictionary.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIPersonalDictionary",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void load (); */
                    Method {
                        name: "Load",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void save (); */
                    Method {
                        name: "Save",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIStringEnumerator wordList; */
                    Method {
                        name: "GetWordList",
                        params: &[Param { name: "aWordList", ty: "*mut*const nsIStringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean check (in AString word); */
                    Method {
                        name: "Check",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addWord (in AString word); */
                    Method {
                        name: "AddWord",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeWord (in AString word); */
                    Method {
                        name: "RemoveWord",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void ignoreWord (in AString word); */
                    Method {
                        name: "IgnoreWord",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void endSession (); */
                    Method {
                        name: "EndSession",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

