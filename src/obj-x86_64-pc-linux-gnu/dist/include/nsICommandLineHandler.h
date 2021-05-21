/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineHandler.idl
 */

#ifndef __gen_nsICommandLineHandler_h__
#define __gen_nsICommandLineHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICommandLine; /* forward declaration */


/* starting interface:    nsICommandLineHandler */
#define NS_ICOMMANDLINEHANDLER_IID_STR "d4b123df-51ee-48b1-a663-002180e60d3b"

#define NS_ICOMMANDLINEHANDLER_IID \
  {0xd4b123df, 0x51ee, 0x48b1, \
    { 0xa6, 0x63, 0x00, 0x21, 0x80, 0xe6, 0x0d, 0x3b }}

class NS_NO_VTABLE nsICommandLineHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDLINEHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandLineHandler;

  /* void handle (in nsICommandLine aCommandLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Handle(nsICommandLine *aCommandLine) = 0;

  /* readonly attribute AUTF8String helpInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHelpInfo(nsACString& aHelpInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandLineHandler, NS_ICOMMANDLINEHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDLINEHANDLER \
  NS_IMETHOD Handle(nsICommandLine *aCommandLine) override; \
  NS_IMETHOD GetHelpInfo(nsACString& aHelpInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDLINEHANDLER \
  nsresult Handle(nsICommandLine *aCommandLine); \
  nsresult GetHelpInfo(nsACString& aHelpInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDLINEHANDLER(_to) \
  NS_IMETHOD Handle(nsICommandLine *aCommandLine) override { return _to Handle(aCommandLine); } \
  NS_IMETHOD GetHelpInfo(nsACString& aHelpInfo) override { return _to GetHelpInfo(aHelpInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDLINEHANDLER(_to) \
  NS_IMETHOD Handle(nsICommandLine *aCommandLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Handle(aCommandLine); } \
  NS_IMETHOD GetHelpInfo(nsACString& aHelpInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHelpInfo(aHelpInfo); } 


#endif /* __gen_nsICommandLineHandler_h__ */
