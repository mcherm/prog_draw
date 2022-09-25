//
// Contains a function to render a CapabilityData as an HTML div (for a pop-up).
//

use crate::capability_tree::CapabilityData;
use html_escape::encode_text;


/// Render the CapabilityData as an HTML div suitable for a pop-up.
#[allow(dead_code)] // this IS used, but from javascript
pub fn as_html(data: &CapabilityData) -> String {
    let name = encode_text(&data.text);
    let description = encode_text(&data.description);
    let notes = encode_text(&data.notes);
    let core_surround: &str = data.core_surround.into();
    let used_by_consumer: &str = data.used_by_set.consumer.into();
    let used_by_sbb: &str = data.used_by_set.sbb.into();
    let used_by_commercial: &str = data.used_by_set.commercial.into();
    format!(
        r##"
            <div class="modal-shade" id="capability_modal" onclick="document.getElementById('capability_modal').remove()">
                <div class="modal-content">
                    <div class="capability_data">
                        <div class="name">{name}</div>
                        <div class="description">
                            <label>Description:</label>
                            <div>{description}</div>
                        </div>
                        <div class="notes">
                            <label>Notes:</label>
                            <div>{notes}</div>
                        </div>
                        <div class="core_surround"><label>Core/Surround: </label>{core_surround}</div>
                        <div class="used_by">
                            <label>Used By:</label>
                            <div class="used_by_grid">
                                <div><label>Consumer</label></div>
                                <div>{used_by_consumer}</div>
                                <div><label>SBB</label></div>
                                <div>{used_by_sbb}</div>
                                <div><label>Commercial</label></div>
                                <div>{used_by_commercial}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        "##,
    ).to_string()
}

/// Return the style tag needed to properly display capability data.
#[allow(dead_code)] // this IS used, but from javascript
pub fn style() -> &'static str {
    r##"
        .modal-shade {
            position: fixed;
            z-index: 1;
            left: 0;
            top: 0;
            width: 100%;
            height: 100%;
            overflow: auto;
            background-color: #00000080;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        .modal-content {
            background-color: #F7F7F7;
            padding: 6px;
            border: 3px solid #000000;
            overflow: scroll;
            max-height: 90%;
            max-width: 90%;
        }            .capability_data {
            max-width: 420px;
            font-family: Arial, sans-serif;
            font-size: 16px;
            border: 2px solid #000000;
            padding: 5px;
        }
        .capability_data label {
            font-weight: bold;
        }
        .capability_data > div {
            margin: 10px 2px;
        }
        .capability_data .name {
            font-size: 20px;
            font-weight: bold;
        }
        .used_by_grid {
            display: inline-grid;
            grid-template-columns: max-content max-content;
            grid-gap: 0;
            border-right: 1px solid #000000;
            border-bottom: 1px solid #000000;
        }
        .used_by_grid > div {
            border-top: 1px solid #000000;
            border-left: 1px solid #000000;
            padding: 2px;
            margin: 0;
        }
"##
}