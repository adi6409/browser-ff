//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISupportsIterators.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOutputIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "PutElement",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "StepForward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIInputIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "GetElement",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "StepForward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "IsEqualTo",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIForwardIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "GetElement",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "PutElement",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "StepForward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "IsEqualTo",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBidirectionalIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "GetElement",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "PutElement",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "StepForward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepBackward (); */
                    Method {
                        name: "StepBackward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "IsEqualTo",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIRandomAccessIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "GetElement",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getElementAt (in int32_t anOffset); */
                    Method {
                        name: "GetElementAt",
                        params: &[Param { name: "anOffset", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "PutElement",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void putElementAt (in int32_t anOffset, in nsISupports anElementToPut); */
                    Method {
                        name: "PutElementAt",
                        params: &[Param { name: "anOffset", ty: "int32_t" }, Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "StepForward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepForwardBy (in int32_t anOffset); */
                    Method {
                        name: "StepForwardBy",
                        params: &[Param { name: "anOffset", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepBackward (); */
                    Method {
                        name: "StepBackward",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void stepBackwardBy (in int32_t anOffset); */
                    Method {
                        name: "StepBackwardBy",
                        params: &[Param { name: "anOffset", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "IsEqualTo",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

