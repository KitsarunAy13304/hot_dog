#![allow(non_snake_case)]
use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // นำเข้าฟอนต์ภาษาไทยเพื่อความมินิมอล (Prompt & Sarabun)
        document::Link { 
            rel: "stylesheet", 
            href: "https://fonts.googleapis.com/css2?family=Prompt:wght@300;400;500;600&family=Sarabun:ital,wght@0,300;0,400;1,400&display=swap" 
        }
        
        // เราสามารถใช้ script ของ Tailwind CDN เพื่อให้มั่นใจว่า UI แสดงผล 100% (เผื่อระบบ build css ยังไม่ได้ตั้งค่า)
        document::Script { src: "https://cdn.tailwindcss.com" }

        div { class: "min-h-screen bg-[#Faf8f5] text-stone-800 font-['Prompt'] selection:bg-amber-200",
            // ---------------- NAVBAR ----------------
            nav { class: "flex items-center justify-between px-6 md:px-12 py-6 max-w-7xl mx-auto",
                div { class: "flex items-center gap-3",
                    // โลโก้จำลอง
                    div { class: "w-10 h-10 rounded-full border-2 border-amber-700 flex items-center justify-center text-amber-700",
                        "🌿"
                    }
                    div { class: "text-2xl font-semibold tracking-wide text-amber-800",
                        "พุทธโอสถ"
                    }
                }
                div { class: "hidden md:flex space-x-10 text-stone-600 text-sm tracking-wider",
                    a { class: "hover:text-amber-700 transition duration-300", href: "#", "หน้าแรก" }
                    a { class: "hover:text-amber-700 transition duration-300", href: "#", "บริการ" }
                    a { class: "hover:text-amber-700 transition duration-300", href: "#", "นัดหมาย" }
                }
            }

            // ---------------- HERO SECTION ----------------
            main { class: "max-w-5xl mx-auto px-6 py-24 md:py-32 flex flex-col items-center text-center",
                div { class: "inline-block mb-6 px-5 py-2 rounded-full border border-amber-200 bg-amber-50/50 text-amber-800 text-sm font-medium tracking-wide",
                    "คลินิกการแพทย์แผนไทย"
                }
                h1 { class: "text-4xl md:text-6xl font-medium mb-8 text-stone-900 leading-tight",
                    "ผสานศาสตร์แห่งภูมิปัญญา"
                    br {}
                    span { class: "text-amber-700 font-['Sarabun'] italic", "สู่ความสมดุลของร่างกาย" }
                }
                p { class: "max-w-2xl text-stone-500 text-lg md:text-xl mb-12 leading-relaxed font-light",
                    "ดูแลสุขภาพของคุณด้วยศาสตร์เวชกรรมไทยประยุกต์ พร้อมเทคโนโลยีวิเคราะห์โครงสร้างและการเคลื่อนไหวที่แม่นยำ เพื่อการรักษาที่ตรงจุดและยั่งยืน"
                }
                button { class: "bg-amber-800 hover:bg-stone-900 text-white px-10 py-4 rounded shadow-sm transition-all duration-500 text-lg tracking-wide",
                    "นัดหมายเข้ารับบริการ"
                }
            }

            // ---------------- SERVICES SECTION ----------------
            section { class: "bg-white py-24 border-t border-stone-100",
                div { class: "max-w-6xl mx-auto px-6 grid grid-cols-1 md:grid-cols-3 gap-16 md:gap-12",
                    // Card 1
                    div { class: "flex flex-col items-center text-center space-y-5 group",
                        div { class: "w-16 h-16 rounded-2xl bg-[#Faf8f5] group-hover:bg-amber-50 transition duration-300 flex items-center justify-center text-amber-700 text-3xl shadow-sm border border-stone-100",
                            "💊"
                        }
                        h3 { class: "text-xl font-medium text-stone-800", "เวชกรรมไทย" }
                        p { class: "text-stone-500 leading-relaxed text-sm font-light", 
                            "ตรวจวินิจฉัยและจ่ายยาสมุนไพร ปรับสมดุลธาตุทั้งสี่ตามคัมภีร์แพทย์แผนไทยดั้งเดิม" 
                        }
                    }
                    // Card 2
                    div { class: "flex flex-col items-center text-center space-y-5 group",
                        div { class: "w-16 h-16 rounded-2xl bg-[#Faf8f5] group-hover:bg-amber-50 transition duration-300 flex items-center justify-center text-amber-700 text-3xl shadow-sm border border-stone-100",
                            "🧘"
                        }
                        h3 { class: "text-xl font-medium text-stone-800", "หัตถเวชกรรมประยุกต์" }
                        p { class: "text-stone-500 leading-relaxed text-sm font-light", 
                            "นวดรักษาแก้อาการ ผสานการประเมินทางชีวกลศาสตร์ (Biomechanics) เพื่อปรับแก้โครงสร้าง" 
                        }
                    }
                    // Card 3
                    div { class: "flex flex-col items-center text-center space-y-5 group",
                        div { class: "w-16 h-16 rounded-2xl bg-[#Faf8f5] group-hover:bg-amber-50 transition duration-300 flex items-center justify-center text-amber-700 text-3xl shadow-sm border border-stone-100",
                            "🏥"
                        }
                        h3 { class: "text-xl font-medium text-stone-800", "คลินิกหนองเสือ" }
                        p { class: "text-stone-500 leading-relaxed text-sm font-light", 
                            "ให้บริการ ณ อ.หนองเสือ จ.ปทุมธานี ในบรรยากาศที่สงบ เป็นส่วนตัว และผ่อนคลาย" 
                        }
                    }
                }
            }

            // ---------------- FOOTER ----------------
            footer { class: "bg-stone-900 text-stone-400 py-12 text-center",
                p { class: "text-sm font-light tracking-wide", "© 2026 Phuttha Osot Clinic. All rights reserved." }
            }
        }
    }
}