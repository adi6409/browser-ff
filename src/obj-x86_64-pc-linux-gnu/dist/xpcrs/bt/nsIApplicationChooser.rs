//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIApplicationChooser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationChooserFinishedCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void done (in nsIHandlerApp handlerApp); */
                    Method {
                        name: "Done",
                        params: &[Param { name: "handlerApp", ty: "*const nsIHandlerApp" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIApplicationChooser",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in mozIDOMWindowProxy parent, in ACString title); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "title", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void open (in ACString contentType, in nsIApplicationChooserFinishedCallback applicationChooserFinishedCallback); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "contentType", ty: "*const ::nsstring::nsACString" }, Param { name: "applicationChooserFinishedCallback", ty: "*const nsIApplicationChooserFinishedCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

