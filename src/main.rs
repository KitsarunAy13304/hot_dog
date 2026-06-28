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
                div { class: "flex items-center gap-4",
                    // โลโก้คลินิก (ใช้รูป logo.jpg จากโฟลเดอร์ assets)
                    img {
                        src: asset!("/assets/logo.jpeg"),
                        alt: "โลโก้พุทธโอสถ",
                        // ปรับขนาด (w-12 h-12) ทำให้เป็นวงกลม (rounded-full) พร้อมมีขอบบางๆ สีเข้าธีม
                        class: "w-12 h-12 rounded-full object-cover shadow-sm border-2 border-amber-700/20"
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
            section { class: "max-w-7xl mx-auto px-6 md:px-12 py-16 md:py-24 flex flex-col items-center text-center",
                h1 { class: "text-4xl md:text-5xl lg:text-6xl font-semibold text-stone-800 leading-tight mb-6",
                    "ฟื้นฟูร่างกายและจิตใจ",
                    br {}
                    "ด้วยวิถีการแพทย์แผนไทยประยุกต์"
                }
                p { class: "text-stone-500 text-lg md:text-xl max-w-2xl mb-10 font-light leading-relaxed",
                    "คลินิกพุทธโอสถ พร้อมดูแลคุณด้วยศาสตร์การแพทย์แผนไทยประยุกต์ ให้คุณกลับมามีสุขภาพที่ดีอย่างยั่งยืน"
                }
                button { class: "bg-amber-700 hover:bg-amber-800 text-white px-8 py-4 rounded-full font-medium tracking-wide transition duration-300 shadow-md",
                    "นัดหมายเข้ารับบริการ"
                }
            }

            // ---------------- SERVICES SECTION ----------------
            section { class: "max-w-7xl mx-auto px-6 md:px-12 py-16 md:py-24",
                div { class: "text-center mb-16",
                    h2 { class: "text-3xl font-semibold text-stone-800 mb-4", "บริการของเรา" }
                    div { class: "w-16 h-1 bg-amber-700 mx-auto rounded-full" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-12",
                    // Card 1
                    div { class: "flex flex-col items-center text-center space-y-5 group",
                        div { class: "w-16 h-16 rounded-2xl bg-[#Faf8f5] group-hover:bg-amber-50 transition duration-300 flex items-center justify-center text-amber-700 text-3xl shadow-sm border border-stone-100",
                            "🌿"
                        }
                        h3 { class: "text-xl font-medium text-stone-800", "เวชกรรมไทย" }
                        p { class: "text-stone-500 leading-relaxed text-sm font-light", 
                            "ตรวจวินิจฉัยและรักษาโรคด้วยยาสมุนไพรตำรับแผนไทยที่ได้มาตรฐาน" 
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
                p { class: "text-sm", "© 2026 พุทธโอสถ คลินิกการแพทย์แผนไทยประยุกต์. All rights reserved." }
            }
        }
    }
}