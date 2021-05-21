/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIControllers.idl
 */

#ifndef __gen_nsIControllers_h__
#define __gen_nsIControllers_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIController; /* forward declaration */

class nsIDOMXULCommandDispatcher; /* forward declaration */


/* starting interface:    nsIControllers */
#define NS_ICONTROLLERS_IID_STR "f36e3ec1-9197-4ad8-8d4c-d3b1927fd6df"

#define NS_ICONTROLLERS_IID \
  {0xf36e3ec1, 0x9197, 0x4ad8, \
    { 0x8d, 0x4c, 0xd3, 0xb1, 0x92, 0x7f, 0xd6, 0xdf }}

class NS_NO_VTABLE nsIControllers : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTROLLERS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIControllers;

  /* nsIController getControllerForCommand (in string command); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) = 0;

  /* void insertControllerAt (in unsigned long index, in nsIController controller); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InsertControllerAt(uint32_t index, nsIController *controller) = 0;

  /* nsIController removeControllerAt (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveControllerAt(uint32_t index, nsIController **_retval) = 0;

  /* nsIController getControllerAt (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerAt(uint32_t index, nsIController **_retval) = 0;

  /* void appendController (in nsIController controller); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendController(nsIController *controller) = 0;

  /* void removeController (in nsIController controller); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveController(nsIController *controller) = 0;

  /* unsigned long getControllerId (in nsIController controller); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerId(nsIController *controller, uint32_t *_retval) = 0;

  /* nsIController getControllerById (in unsigned long controllerID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerById(uint32_t controllerID, nsIController **_retval) = 0;

  /* unsigned long getControllerCount (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerCount(uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIControllers, NS_ICONTROLLERS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTROLLERS \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override; \
  NS_IMETHOD InsertControllerAt(uint32_t index, nsIController *controller) override; \
  NS_IMETHOD RemoveControllerAt(uint32_t index, nsIController **_retval) override; \
  NS_IMETHOD GetControllerAt(uint32_t index, nsIController **_retval) override; \
  NS_IMETHOD AppendController(nsIController *controller) override; \
  NS_IMETHOD RemoveController(nsIController *controller) override; \
  NS_IMETHOD GetControllerId(nsIController *controller, uint32_t *_retval) override; \
  NS_IMETHOD GetControllerById(uint32_t controllerID, nsIController **_retval) override; \
  NS_IMETHOD GetControllerCount(uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTROLLERS \
  nsresult GetControllerForCommand(const char * command, nsIController **_retval); \
  nsresult InsertControllerAt(uint32_t index, nsIController *controller); \
  nsresult RemoveControllerAt(uint32_t index, nsIController **_retval); \
  nsresult GetControllerAt(uint32_t index, nsIController **_retval); \
  nsresult AppendController(nsIController *controller); \
  nsresult RemoveController(nsIController *controller); \
  nsresult GetControllerId(nsIController *controller, uint32_t *_retval); \
  nsresult GetControllerById(uint32_t controllerID, nsIController **_retval); \
  nsresult GetControllerCount(uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTROLLERS(_to) \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override { return _to GetControllerForCommand(command, _retval); } \
  NS_IMETHOD InsertControllerAt(uint32_t index, nsIController *controller) override { return _to InsertControllerAt(index, controller); } \
  NS_IMETHOD RemoveControllerAt(uint32_t index, nsIController **_retval) override { return _to RemoveControllerAt(index, _retval); } \
  NS_IMETHOD GetControllerAt(uint32_t index, nsIController **_retval) override { return _to GetControllerAt(index, _retval); } \
  NS_IMETHOD AppendController(nsIController *controller) override { return _to AppendController(controller); } \
  NS_IMETHOD RemoveController(nsIController *controller) override { return _to RemoveController(controller); } \
  NS_IMETHOD GetControllerId(nsIController *controller, uint32_t *_retval) override { return _to GetControllerId(controller, _retval); } \
  NS_IMETHOD GetControllerById(uint32_t controllerID, nsIController **_retval) override { return _to GetControllerById(controllerID, _retval); } \
  NS_IMETHOD GetControllerCount(uint32_t *_retval) override { return _to GetControllerCount(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTROLLERS(_to) \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerForCommand(command, _retval); } \
  NS_IMETHOD InsertControllerAt(uint32_t index, nsIController *controller) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertControllerAt(index, controller); } \
  NS_IMETHOD RemoveControllerAt(uint32_t index, nsIController **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveControllerAt(index, _retval); } \
  NS_IMETHOD GetControllerAt(uint32_t index, nsIController **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerAt(index, _retval); } \
  NS_IMETHOD AppendController(nsIController *controller) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendController(controller); } \
  NS_IMETHOD RemoveController(nsIController *controller) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveController(controller); } \
  NS_IMETHOD GetControllerId(nsIController *controller, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerId(controller, _retval); } \
  NS_IMETHOD GetControllerById(uint32_t controllerID, nsIController **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerById(controllerID, _retval); } \
  NS_IMETHOD GetControllerCount(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerCount(_retval); } 


#endif /* __gen_nsIControllers_h__ */
