# Design System Strategy: The Forest Editorial



## 1. Overview & Creative North Star

**Creative North Star: "The Analog Naturalist"**



This design system rejects the clinical, high-contrast rigidity of modern SaaS platforms in favor of a sophisticated, editorial experience inspired by high-end botanical journals and architectural archives. It is a "Low-Contrast, High-Intent" system that prioritizes atmospheric depth over artificial sharpness.



To move beyond a "standard" UI, we utilize **Intentional Asymmetry**. Instead of perfectly centered grids, we use the Spacing Scale to create "breathing gutters"—large, unexpected gaps of negative space that allow the 'Hard' forest backgrounds to feel like a physical environment rather than just a backdrop. We break the "template" look by overlapping typography across container boundaries and using tonal shifts rather than lines to define the viewport.



---



## 2. Colors

The palette is a curated study of the Everforest aesthetic, moving from deep, "Hard" forest greens to luminous, "Earth" accents.



### Surface Hierarchy & Nesting

We achieve depth through a "Soil to Canopy" layering logic. Instead of shadows, we use the `surface-container` tiers to represent physical elevation.

- **Base Layer:** `surface` (#0c141a) or `surface-dim`.

- **Primary Content Area:** `surface-container-low` (#151d22).

- **Interactive Floating Elements:** `surface-container-highest` (#2e363c).



### The "No-Line" Rule

**Explicit Instruction:** Do not use 1px solid borders to section off content.

Boundaries must be defined solely through background color shifts. For example, a navigation sidebar should be `surface-container-lowest` sitting against a `surface` main content area. If the transition feels too soft, increase the spacing—do not add a line.



### The "Glass & Gradient" Rule

To elevate the UI from "flat" to "bespoke," use **Glassmorphism** for floating headers or modals.

- **Recipe:** Apply `surface-container` at 70% opacity with a `20px` to `40px` backdrop-blur.

- **Signature Textures:** For high-impact CTAs, use a subtle linear gradient from `primary` (#c2dc99) to `primary-container` (#a7c080) at a 135-degree angle. This mimics the way light hits a leaf, providing a "soul" that flat hex codes cannot achieve.



---



## 3. Typography

The system utilizes a dual-typeface approach to balance technical precision with organic warmth.



* **Display & Headlines (Space Grotesk):** An eccentric, semi-monospaced feel that provides an "authoritative editorial" voice. Use `display-lg` for hero moments, intentionally reducing letter-spacing by `-0.02em` to create a tight, professional block of text.

* **Body & Titles (Manrope):** A highly legible geometric sans-serif that softens the "technical" edge of Space Grotesk. Manrope handles the heavy lifting of information density.



**Hierarchy as Identity:**

- **The "Over-Sized" Header:** Use `display-lg` alongside `body-sm` to create extreme scale contrast. This high-low pairing is the hallmark of high-end digital editorial design.

- **Labels:** Use `label-md` in all-caps with `0.05em` letter-spacing for metadata, rendered in `secondary` (#ffb595) to act as a visual anchor.



---



## 4. Elevation & Depth

In this design system, light does not come from a "drop shadow"—it comes from the material itself.



* **The Layering Principle:** Place a `surface-container-lowest` card on a `surface-container-low` section. This creates a "recessed" look, as if the content is carved into the interface.

* **Ambient Shadows:** If a floating element (like a dropdown) requires a shadow, use a large blur (30px+) at 8% opacity. Use a tinted shadow: `rgba(7, 15, 20, 0.5)` rather than pure black.

* **The "Ghost Border" Fallback:** If accessibility requires a container edge, use a "Ghost Border": `outline-variant` (#45483d) at **15% opacity**. It should be felt, not seen.

* **Soft Roundness:** Stick to the `md` (0.75rem) and `lg` (1rem) tokens. Avoid the `full` pill-shape for anything other than tags; we want the UI to feel like "rounded stone," not "plastic capsules."



---



## 5. Components



### Buttons

- **Primary:** Gradient of `primary` to `primary-container`. Text in `on-primary`. Corner radius `md`.

- **Secondary:** `surface-container-highest` background with `primary` text. No border.

- **Tertiary:** No background. `primary` text with an underline that appears only on hover.



### Input Fields

Avoid the "boxed" look. Use `surface-container-highest` as the background with a `sm` (0.25rem) bottom-only radius to create a "tab" feel. Use `primary` for the cursor and active-state bottom-indicator (2px).



### Cards & Lists

**Forbid divider lines.** Separate list items using the `2` (0.5rem) spacing token and a slight background shift on hover. For cards, use `surface-container-low` and rely on the `lg` (1rem) padding to create internal hierarchy.



### Signature Component: The "Environmental Insight"

A bespoke component for this system: A large-format decorative block using a `tertiary-container` (#d2b478) background, `headline-sm` typography, and 15% opacity `outline` Ghost Borders. Use this for pull-quotes or key feature highlights.



---



## 6. Do’s and Don’ts



### Do

- **Do** use `20` (5rem) and `24` (6rem) spacing tokens between major sections to emphasize the "Forest" openness.

- **Do** use `secondary` (Orange/Earth) sparingly as a "Notification" or "Critical Action" color; it should pop against the green.

- **Do** overlap images or text elements across container edges to break the "web-page" feel.



### Don’t

- **Don’t** use pure white (#FFFFFF) for text. Always use `on-surface` (#dbe3eb) to maintain the soft, analog atmosphere.

- **Don’t** use 100% opaque borders. They create "visual noise" that destroys the organic flow of the Everforest palette.

- **Don’t** use standard "Material Design" shadows. They are too aggressive for this palette's tonal depth.
