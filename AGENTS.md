# Project guidelines

- Keep this project independent from Better Somniel; no settings, resource edits or custom save data.
- Preserve hair, weapons, mounts, animation compatibility and engaged forms.
- Engage and Engage+ opening actors use the character's original dress before transformation. Identify the opening through the native condition literal used by EngageSequence.Cast.Import, not localized UI text or an Engage substring. Keep transformed states, GodUnit models and animation/effect fields native.
- Use apply_patch for edits, keep commits atomic and do not install or release without an explicit request.
- Keep README focused on this mod, retaining Credits and License.
- Verify target signatures against the read-only Engage 2.0.0 NSO and distinguish modeled checks from in-game testing.
