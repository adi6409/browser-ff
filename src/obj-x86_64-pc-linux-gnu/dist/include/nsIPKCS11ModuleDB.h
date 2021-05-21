/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11ModuleDB.idl
 */

#ifndef __gen_nsIPKCS11ModuleDB_h__
#define __gen_nsIPKCS11ModuleDB_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPKCS11Module; /* forward declaration */

class nsIPKCS11Slot; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

#define NS_PKCS11MODULEDB_CONTRACTID "@mozilla.org/security/pkcs11moduledb;1"

/* starting interface:    nsIPKCS11ModuleDB */
#define NS_IPKCS11MODULEDB_IID_STR "ff9fbcd7-9517-4334-b97a-ceed78909974"

#define NS_IPKCS11MODULEDB_IID \
  {0xff9fbcd7, 0x9517, 0x4334, \
    { 0xb9, 0x7a, 0xce, 0xed, 0x78, 0x90, 0x99, 0x74 }}

class NS_NO_VTABLE nsIPKCS11ModuleDB : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPKCS11MODULEDB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPKCS11ModuleDB;

  /* [must_use] void deleteModule (in AString moduleName); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD DeleteModule(const nsAString& moduleName) = 0;

  /* [must_use] void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AddModule(const nsAString& moduleName, const nsAString& libraryFullPath, int32_t cryptoMechanismFlags, int32_t cipherFlags) = 0;

  /* [must_use] nsISimpleEnumerator listModules (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ListModules(nsISimpleEnumerator **_retval) = 0;

  /* [must_use] readonly attribute boolean canToggleFIPS; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCanToggleFIPS(bool *aCanToggleFIPS) = 0;

  /* [must_use] void toggleFIPSMode (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ToggleFIPSMode(void) = 0;

  /* [must_use] readonly attribute boolean isFIPSEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetIsFIPSEnabled(bool *aIsFIPSEnabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPKCS11ModuleDB, NS_IPKCS11MODULEDB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPKCS11MODULEDB \
  [[nodiscard]] NS_IMETHOD DeleteModule(const nsAString& moduleName) override; \
  [[nodiscard]] NS_IMETHOD AddModule(const nsAString& moduleName, const nsAString& libraryFullPath, int32_t cryptoMechanismFlags, int32_t cipherFlags) override; \
  [[nodiscard]] NS_IMETHOD ListModules(nsISimpleEnumerator **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetCanToggleFIPS(bool *aCanToggleFIPS) override; \
  [[nodiscard]] NS_IMETHOD ToggleFIPSMode(void) override; \
  [[nodiscard]] NS_IMETHOD GetIsFIPSEnabled(bool *aIsFIPSEnabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPKCS11MODULEDB \
  [[nodiscard]] nsresult DeleteModule(const nsAString& moduleName); \
  [[nodiscard]] nsresult AddModule(const nsAString& moduleName, const nsAString& libraryFullPath, int32_t cryptoMechanismFlags, int32_t cipherFlags); \
  [[nodiscard]] nsresult ListModules(nsISimpleEnumerator **_retval); \
  [[nodiscard]] nsresult GetCanToggleFIPS(bool *aCanToggleFIPS); \
  [[nodiscard]] nsresult ToggleFIPSMode(void); \
  [[nodiscard]] nsresult GetIsFIPSEnabled(bool *aIsFIPSEnabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPKCS11MODULEDB(_to) \
  [[nodiscard]] NS_IMETHOD DeleteModule(const nsAString& moduleName) override { return _to DeleteModule(moduleName); } \
  [[nodiscard]] NS_IMETHOD AddModule(const nsAString& moduleName, const nsAString& libraryFullPath, int32_t cryptoMechanismFlags, int32_t cipherFlags) override { return _to AddModule(moduleName, libraryFullPath, cryptoMechanismFlags, cipherFlags); } \
  [[nodiscard]] NS_IMETHOD ListModules(nsISimpleEnumerator **_retval) override { return _to ListModules(_retval); } \
  [[nodiscard]] NS_IMETHOD GetCanToggleFIPS(bool *aCanToggleFIPS) override { return _to GetCanToggleFIPS(aCanToggleFIPS); } \
  [[nodiscard]] NS_IMETHOD ToggleFIPSMode(void) override { return _to ToggleFIPSMode(); } \
  [[nodiscard]] NS_IMETHOD GetIsFIPSEnabled(bool *aIsFIPSEnabled) override { return _to GetIsFIPSEnabled(aIsFIPSEnabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPKCS11MODULEDB(_to) \
  [[nodiscard]] NS_IMETHOD DeleteModule(const nsAString& moduleName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteModule(moduleName); } \
  [[nodiscard]] NS_IMETHOD AddModule(const nsAString& moduleName, const nsAString& libraryFullPath, int32_t cryptoMechanismFlags, int32_t cipherFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddModule(moduleName, libraryFullPath, cryptoMechanismFlags, cipherFlags); } \
  [[nodiscard]] NS_IMETHOD ListModules(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ListModules(_retval); } \
  [[nodiscard]] NS_IMETHOD GetCanToggleFIPS(bool *aCanToggleFIPS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanToggleFIPS(aCanToggleFIPS); } \
  [[nodiscard]] NS_IMETHOD ToggleFIPSMode(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToggleFIPSMode(); } \
  [[nodiscard]] NS_IMETHOD GetIsFIPSEnabled(bool *aIsFIPSEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFIPSEnabled(aIsFIPSEnabled); } 


#endif /* __gen_nsIPKCS11ModuleDB_h__ */
