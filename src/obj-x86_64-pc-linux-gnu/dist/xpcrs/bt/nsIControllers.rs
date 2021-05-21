//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIControllers.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllers",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIController getControllerForCommand (in string command); */
                    Method {
                        name: "GetControllerForCommand",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void insertControllerAt (in unsigned long index, in nsIController controller); */
                    Method {
                        name: "InsertControllerAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "controller", ty: "*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIController removeControllerAt (in unsigned long index); */
                    Method {
                        name: "RemoveControllerAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIController getControllerAt (in unsigned long index); */
                    Method {
                        name: "GetControllerAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendController (in nsIController controller); */
                    Method {
                        name: "AppendController",
                        params: &[Param { name: "controller", ty: "*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeController (in nsIController controller); */
                    Method {
                        name: "RemoveController",
                        params: &[Param { name: "controller", ty: "*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long getControllerId (in nsIController controller); */
                    Method {
                        name: "GetControllerId",
                        params: &[Param { name: "controller", ty: "*const nsIController" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIController getControllerById (in unsigned long controllerID); */
                    Method {
                        name: "GetControllerById",
                        params: &[Param { name: "controllerID", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long getControllerCount (); */
                    Method {
                        name: "GetControllerCount",
                        params: &[Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

