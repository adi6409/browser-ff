#include "AudioWorkletGlobalScopeBinding.h"
#include "AudioWorkletProcessorBinding.h"
#include "ConsoleBinding.h"
#include "EventTargetBinding.h"
#include "MessagePortBinding.h"
#include "PaintWorkletGlobalScopeBinding.h"
#include "WorkletGlobalScopeBinding.h"

namespace mozilla {
namespace dom {
bool
RegisterWorkletBindings(JSContext* aCx, JS::Handle<JSObject*> aObj)
{
  if (!AudioWorkletGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!AudioWorkletProcessor_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ConsoleInstance_Binding::ConstructorEnabled(aCx, aObj) && !ConsoleInstance_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!EventTarget_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!MessagePort_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PaintWorkletGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WorkletGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!console_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  return true;
}

} // namespace dom
} // namespace mozilla

