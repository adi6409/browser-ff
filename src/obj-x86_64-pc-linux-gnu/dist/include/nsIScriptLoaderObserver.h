/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/script/nsIScriptLoaderObserver.idl
 */

#ifndef __gen_nsIScriptLoaderObserver_h__
#define __gen_nsIScriptLoaderObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIScriptElement; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIScriptLoaderObserver */
#define NS_ISCRIPTLOADEROBSERVER_IID_STR "7b787204-76fb-4764-96f1-fb7a666db4f4"

#define NS_ISCRIPTLOADEROBSERVER_IID \
  {0x7b787204, 0x76fb, 0x4764, \
    { 0x96, 0xf1, 0xfb, 0x7a, 0x66, 0x6d, 0xb4, 0xf4 }}

class NS_NO_VTABLE nsIScriptLoaderObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTLOADEROBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptLoaderObserver;

  /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInlineClassicScript, in nsIURI aURI, in int32_t aLineNo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScriptAvailable(nsresult aResult, nsIScriptElement *aElement, bool aIsInlineClassicScript, nsIURI *aURI, int32_t aLineNo) = 0;

  /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScriptEvaluated(nsresult aResult, nsIScriptElement *aElement, bool aIsInline) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptLoaderObserver, NS_ISCRIPTLOADEROBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTLOADEROBSERVER \
  NS_IMETHOD ScriptAvailable(nsresult aResult, nsIScriptElement *aElement, bool aIsInlineClassicScript, nsIURI *aURI, int32_t aLineNo) override; \
  NS_IMETHOD ScriptEvaluated(nsresult aResult, nsIScriptElement *aElement, bool aIsInline) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTLOADEROBSERVER \
  nsresult ScriptAvailable(nsresult aResult, nsIScriptElement *aElement, bool aIsInlineClassicScript, nsIURI *aURI, int32_t aLineNo); \
  nsresult ScriptEvaluated(nsresult aResult, nsIScriptElement *aElement, bool aIsInline); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTLOADEROBSERVER(_to) \
  NS_IMETHOD ScriptAvailable(nsresult aResult, nsIScriptElement *aElement, bool aIsInlineClassicScript, nsIURI *aURI, int32_t aLineNo) override { return _to ScriptAvailable(aResult, aElement, aIsInlineClassicScript, aURI, aLineNo); } \
  NS_IMETHOD ScriptEvaluated(nsresult aResult, nsIScriptElement *aElement, bool aIsInline) override { return _to ScriptEvaluated(aResult, aElement, aIsInline); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTLOADEROBSERVER(_to) \
  NS_IMETHOD ScriptAvailable(nsresult aResult, nsIScriptElement *aElement, bool aIsInlineClassicScript, nsIURI *aURI, int32_t aLineNo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScriptAvailable(aResult, aElement, aIsInlineClassicScript, aURI, aLineNo); } \
  NS_IMETHOD ScriptEvaluated(nsresult aResult, nsIScriptElement *aElement, bool aIsInline) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScriptEvaluated(aResult, aElement, aIsInline); } 


#endif /* __gen_nsIScriptLoaderObserver_h__ */
