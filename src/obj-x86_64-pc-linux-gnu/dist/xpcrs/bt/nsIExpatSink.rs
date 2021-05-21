//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/htmlparser/nsIExpatSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExpatSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber, in unsigned long aColumnNumber); */
                    Method {
                        name: "HandleStartElement",
                        params: &[Param { name: "aName", ty: "*const i16" }, Param { name: "aAtts", ty: "*mut *const i16" }, Param { name: "aAttsCount", ty: "u32" }, Param { name: "aLineNumber", ty: "u32" }, Param { name: "aColumnNumber", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleEndElement (in wstring aName); */
                    Method {
                        name: "HandleEndElement",
                        params: &[Param { name: "aName", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleComment (in wstring aCommentText); */
                    Method {
                        name: "HandleComment",
                        params: &[Param { name: "aCommentText", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
                    Method {
                        name: "HandleCDataSection",
                        params: &[Param { name: "aData", ty: "*const i16" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData); */
                    Method {
                        name: "HandleDoctypeDecl",
                        params: &[Param { name: "aSubset", ty: "*const ::nsstring::nsAString" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aSystemId", ty: "*const ::nsstring::nsAString" }, Param { name: "aPublicId", ty: "*const ::nsstring::nsAString" }, Param { name: "aCatalogData", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
                    Method {
                        name: "HandleCharacterData",
                        params: &[Param { name: "aData", ty: "*const i16" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleProcessingInstruction (in wstring aTarget, in wstring aData); */
                    Method {
                        name: "HandleProcessingInstruction",
                        params: &[Param { name: "aTarget", ty: "*const i16" }, Param { name: "aData", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone); */
                    Method {
                        name: "HandleXMLDeclaration",
                        params: &[Param { name: "aVersion", ty: "*const i16" }, Param { name: "aEncoding", ty: "*const i16" }, Param { name: "aStandalone", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError); */
                    Method {
                        name: "ReportError",
                        params: &[Param { name: "aErrorText", ty: "*const i16" }, Param { name: "aSourceText", ty: "*const i16" }, Param { name: "aError", ty: "*const nsIScriptError" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

