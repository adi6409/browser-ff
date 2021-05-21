/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/extensions/amIWebInstallPrompt.idl
 */

#ifndef __gen_amIWebInstallPrompt_h__
#define __gen_amIWebInstallPrompt_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIVariant; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    amIWebInstallPrompt */
#define AMIWEBINSTALLPROMPT_IID_STR "386906f1-4d18-45bf-bc81-5dcd68e42c3b"

#define AMIWEBINSTALLPROMPT_IID \
  {0x386906f1, 0x4d18, 0x45bf, \
    { 0xbc, 0x81, 0x5d, 0xcd, 0x68, 0xe4, 0x2c, 0x3b }}

class NS_NO_VTABLE amIWebInstallPrompt : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(AMIWEBINSTALLPROMPT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = amIWebInstallPrompt;

  /* void confirm (in Element aBrowser, in nsIURI aUri, in Array<nsIVariant> aInstalls); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Confirm(mozilla::dom::Element *aBrowser, nsIURI *aUri, const nsTArray<RefPtr<nsIVariant>>& aInstalls) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(amIWebInstallPrompt, AMIWEBINSTALLPROMPT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_AMIWEBINSTALLPROMPT \
  NS_IMETHOD Confirm(mozilla::dom::Element *aBrowser, nsIURI *aUri, const nsTArray<RefPtr<nsIVariant>>& aInstalls) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_AMIWEBINSTALLPROMPT \
  nsresult Confirm(mozilla::dom::Element *aBrowser, nsIURI *aUri, const nsTArray<RefPtr<nsIVariant>>& aInstalls); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_AMIWEBINSTALLPROMPT(_to) \
  NS_IMETHOD Confirm(mozilla::dom::Element *aBrowser, nsIURI *aUri, const nsTArray<RefPtr<nsIVariant>>& aInstalls) override { return _to Confirm(aBrowser, aUri, aInstalls); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_AMIWEBINSTALLPROMPT(_to) \
  NS_IMETHOD Confirm(mozilla::dom::Element *aBrowser, nsIURI *aUri, const nsTArray<RefPtr<nsIVariant>>& aInstalls) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Confirm(aBrowser, aUri, aInstalls); } 


#endif /* __gen_amIWebInstallPrompt_h__ */
