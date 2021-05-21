/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerAuthPrompter.idl
 */

#ifndef __gen_nsILoginManagerAuthPrompter_h__
#define __gen_nsILoginManagerAuthPrompter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsILoginInfo; /* forward declaration */

class nsIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsILoginManagerAuthPrompter */
#define NS_ILOGINMANAGERAUTHPROMPTER_IID_STR "425f73b9-b2db-4e8a-88c5-9ac2512934ce"

#define NS_ILOGINMANAGERAUTHPROMPTER_IID \
  {0x425f73b9, 0xb2db, 0x4e8a, \
    { 0x88, 0xc5, 0x9a, 0xc2, 0x51, 0x29, 0x34, 0xce }}

class NS_NO_VTABLE nsILoginManagerAuthPrompter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMANAGERAUTHPROMPTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginManagerAuthPrompter;

  /* void init (in nsIDOMWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIDOMWindow *aWindow) = 0;

  /* attribute Element browser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBrowser(mozilla::dom::Element **aBrowser) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBrowser(mozilla::dom::Element *aBrowser) = 0;

  /* attribute Element openerBrowser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOpenerBrowser(mozilla::dom::Element *aOpenerBrowser) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginManagerAuthPrompter, NS_ILOGINMANAGERAUTHPROMPTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMANAGERAUTHPROMPTER \
  NS_IMETHOD Init(nsIDOMWindow *aWindow) override; \
  NS_IMETHOD GetBrowser(mozilla::dom::Element **aBrowser) override; \
  NS_IMETHOD SetBrowser(mozilla::dom::Element *aBrowser) override; \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override; \
  NS_IMETHOD SetOpenerBrowser(mozilla::dom::Element *aOpenerBrowser) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMANAGERAUTHPROMPTER \
  nsresult Init(nsIDOMWindow *aWindow); \
  nsresult GetBrowser(mozilla::dom::Element **aBrowser); \
  nsresult SetBrowser(mozilla::dom::Element *aBrowser); \
  nsresult GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser); \
  nsresult SetOpenerBrowser(mozilla::dom::Element *aOpenerBrowser); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMANAGERAUTHPROMPTER(_to) \
  NS_IMETHOD Init(nsIDOMWindow *aWindow) override { return _to Init(aWindow); } \
  NS_IMETHOD GetBrowser(mozilla::dom::Element **aBrowser) override { return _to GetBrowser(aBrowser); } \
  NS_IMETHOD SetBrowser(mozilla::dom::Element *aBrowser) override { return _to SetBrowser(aBrowser); } \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override { return _to GetOpenerBrowser(aOpenerBrowser); } \
  NS_IMETHOD SetOpenerBrowser(mozilla::dom::Element *aOpenerBrowser) override { return _to SetOpenerBrowser(aOpenerBrowser); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMANAGERAUTHPROMPTER(_to) \
  NS_IMETHOD Init(nsIDOMWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aWindow); } \
  NS_IMETHOD GetBrowser(mozilla::dom::Element **aBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowser(aBrowser); } \
  NS_IMETHOD SetBrowser(mozilla::dom::Element *aBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBrowser(aBrowser); } \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpenerBrowser(aOpenerBrowser); } \
  NS_IMETHOD SetOpenerBrowser(mozilla::dom::Element *aOpenerBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOpenerBrowser(aOpenerBrowser); } 


#define NS_LOGINMANAGERAUTHPROMPTER_CONTRACTID "@mozilla.org/login-manager/authprompter/;1"

#endif /* __gen_nsILoginManagerAuthPrompter_h__ */
