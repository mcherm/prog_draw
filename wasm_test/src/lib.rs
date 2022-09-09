use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello there, {}!", name));
}


#[wasm_bindgen]
pub fn get_svg() -> String {
    SAMPLE_SVG.into()
}


static SAMPLE_SVG: &str = r##"
<svg viewBox="-522 -336 1044 474" xmlns="http://www.w3.org/2000/svg">
  <g>
    <g transform="translate(0 -250) scale(0.5)">
<g>
  <g>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" fill="#ffc77f"/>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" fill="#ffff7f"/>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" fill="#804000"/>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" fill="#80ff80"/>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" fill="#f58cff"/>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" fill="#8080ff"/>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" fill="#ff6163"/>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <text font-family="Arial" font-size="18" fill="#000000" x="-47.153914" y="-79.83107">Commercial</text>
  <text x="-131.024" y="62.162277" font-family="Arial" font-size="16" fill="#000000">Consumer</text>
  <text font-family="Arial" font-size="16" fill="#000000" x="75.5" y="65.5">SBB</text>
  <text font-family="Arial" font-size="16" fill="#FFFFFF" x="-9.362914" y="17">All</text>
</g>
    </g>
    <g transform="translate(-12,0)">
      <g>
        <style>
          text.leaf {
            pointer-events: none;
          }
        </style>
        <path d="M -4 0 C -14 0, -10 -39, -20 -39" fill="none" stroke="black"/>
        <path d="M -4 0 C -14 0, -10 39, -20 39" fill="none" stroke="black"/>
        <path d="M -146 -39 C -156 -39, -152 -65, -162 -65" fill="none" stroke="black"/>
        <path d="M -146 -39 C -156 -39, -152 -13, -162 -13" fill="none" stroke="black"/>
        <path d="M -280 -65 C -290 -65, -286 -91, -296 -91" fill="none" stroke="black"/>
        <path d="M -280 -65 C -290 -65, -286 -65, -296 -65" fill="none" stroke="black"/>
        <path d="M -280 -65 C -290 -65, -286 -39, -296 -39" fill="none" stroke="black"/>
        <rect x="-451" y="-100" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf" onclick="alert('1')"/>
        <text x="-449" y="-86" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="-468" y="-74" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-466" y="-60" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="-435" y="-48" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-433" y="-34" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="-280" y="-74" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-278" y="-60" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <rect x="-302" y="-22" width="140" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-300" y="-8" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Capital One Legal Entity</text>
        <rect x="-146" y="-48" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-144" y="-34" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <path d="M -125 39 C -135 39, -131 13, -141 13" fill="none" stroke="black"/>
        <path d="M -125 39 C -135 39, -131 65, -141 65" fill="none" stroke="black"/>
        <rect x="-310" y="4" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-308" y="18" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M -253 65 C -263 65, -259 39, -269 39" fill="none" stroke="black"/>
        <path d="M -253 65 C -263 65, -259 65, -269 65" fill="none" stroke="black"/>
        <path d="M -253 65 C -263 65, -259 91, -269 91" fill="none" stroke="black"/>
        <rect x="-522" y="30" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-520" y="44" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="-510" y="56" width="241" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-508" y="70" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Assign funds availability policy to accounts</text>
        <rect x="-404" y="82" width="135" height="18" rx="3" fill="#7F7FFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-402" y="96" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="-253" y="56" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-251" y="70" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <rect x="-125" y="30" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-123" y="44" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
      </g>
    </g>
    <g transform="translate(12,0)">
      <g>
        <style>
        </style>
        <path d="M 4 0 C 14 0, 10 -39, 20 -39" fill="none" stroke="black"/>
        <path d="M 4 0 C 14 0, 10 39, 20 39" fill="none" stroke="black"/>
        <path d="M 146 -39 C 156 -39, 152 -65, 162 -65" fill="none" stroke="black"/>
        <path d="M 146 -39 C 156 -39, 152 -13, 162 -13" fill="none" stroke="black"/>
        <path d="M 280 -65 C 290 -65, 286 -91, 296 -91" fill="none" stroke="black"/>
        <path d="M 280 -65 C 290 -65, 286 -65, 296 -65" fill="none" stroke="black"/>
        <path d="M 280 -65 C 290 -65, 286 -39, 296 -39" fill="none" stroke="black"/>
        <rect x="296" y="-100" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-86" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="296" y="-74" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-60" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="296" y="-48" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-34" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="162" y="-74" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="164" y="-60" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <rect x="162" y="-22" width="140" height="18" rx="3" fill="#FFFF7F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="164" y="-8" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Capital One Legal Entity</text>
        <rect x="20" y="-48" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="-34" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <path d="M 125 39 C 135 39, 131 13, 141 13" fill="none" stroke="black"/>
        <path d="M 125 39 C 135 39, 131 65, 141 65" fill="none" stroke="black"/>
        <rect x="141" y="4" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="143" y="18" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M 253 65 C 263 65, 259 39, 269 39" fill="none" stroke="black"/>
        <path d="M 253 65 C 263 65, 259 65, 269 65" fill="none" stroke="black"/>
        <path d="M 253 65 C 263 65, 259 91, 269 91" fill="none" stroke="black"/>
        <rect x="269" y="30" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="44" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="269" y="56" width="241" height="18" rx="3" fill="#FFFF7F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="70" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Assign funds availability policy to accounts</text>
        <rect x="269" y="82" width="135" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="96" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="141" y="56" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="143" y="70" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <rect x="20" y="30" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="44" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
      </g>
    </g>
    <circle cx="0" cy="0" r="16"/>
  </g>
</svg>
"##;
