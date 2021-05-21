//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/script/nsIScriptLoaderObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptLoaderObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInlineClassicScript, in nsIURI aURI, in int32_t aLineNo); */
                    Method {
                        name: "ScriptAvailable",
                        params: &[Param { name: "aResult", ty: "::nserror::nsresult" }, Param { name: "aElement", ty: "*const nsIScriptElement" }, Param { name: "aIsInlineClassicScript", ty: "bool" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLineNo", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
                    Method {
                        name: "ScriptEvaluated",
                        params: &[Param { name: "aResult", ty: "::nserror::nsresult" }, Param { name: "aElement", ty: "*const nsIScriptElement" }, Param { name: "aIsInline", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

