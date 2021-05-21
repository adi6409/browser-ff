/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipTextProvider.idl
 */

#ifndef __gen_nsITooltipTextProvider_h__
#define __gen_nsITooltipTextProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsINode; /* webidl Node */


/* starting interface:    nsITooltipTextProvider */
#define NS_ITOOLTIPTEXTPROVIDER_IID_STR "b128a1e6-44f3-4331-8fbe-5af360ff21ee"

#define NS_ITOOLTIPTEXTPROVIDER_IID \
  {0xb128a1e6, 0x44f3, 0x4331, \
    { 0x8f, 0xbe, 0x5a, 0xf3, 0x60, 0xff, 0x21, 0xee }}

class NS_NO_VTABLE nsITooltipTextProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLTIPTEXTPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITooltipTextProvider;

  /* boolean getNodeText (in Node aNode, out wstring aText, out wstring aDirection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNodeText(nsINode *aNode, char16_t * *aText, char16_t * *aDirection, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITooltipTextProvider, NS_ITOOLTIPTEXTPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLTIPTEXTPROVIDER \
  NS_IMETHOD GetNodeText(nsINode *aNode, char16_t * *aText, char16_t * *aDirection, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLTIPTEXTPROVIDER \
  nsresult GetNodeText(nsINode *aNode, char16_t * *aText, char16_t * *aDirection, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLTIPTEXTPROVIDER(_to) \
  NS_IMETHOD GetNodeText(nsINode *aNode, char16_t * *aText, char16_t * *aDirection, bool *_retval) override { return _to GetNodeText(aNode, aText, aDirection, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLTIPTEXTPROVIDER(_to) \
  NS_IMETHOD GetNodeText(nsINode *aNode, char16_t * *aText, char16_t * *aDirection, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNodeText(aNode, aText, aDirection, _retval); } 


#endif /* __gen_nsITooltipTextProvider_h__ */
