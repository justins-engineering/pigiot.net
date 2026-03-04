use dioxus::prelude::*;
#[component]
pub fn Navbar() -> Element {
  rsx! {
    html { class: "dark", lang: "en",
        head {
            meta { charset: "UTF-8" }
            meta {
                content: "width=device-width, initial-scale=1.0",
                name: "viewport",
            }
            title { "Pigeon IoT | Secure, Efficient Rust-Powered IoT Platform" }
            script { src: "https://cdn.tailwindcss.com" }
            script {
                defer: "false",
                src: "https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js",
            }
            link {
                href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css",
                rel: "stylesheet",
            }
            link { href: "https://fonts.googleapis.com", rel: "preconnect" }
            link {
                crossorigin: "false",
                href: "https://fonts.gstatic.com",
                rel: "preconnect",
            }
            link {
                href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&display=swap",
                rel: "stylesheet",
            }
            script {
                "tailwind.config = {darkMode: 'class',theme: {    extend: {        colors: {            teal: {                400: '#2dd4bf',                500: '#14b8a6',                600: '#0d9488',                700: '#0f766e',                800: '#115e59',            },            purple: {                400: '#a855f7',                500: '#8b5cf6',                600: '#7c3aed',                700: '#6d28d9',                800: '#5b21b6',            },            stone: {                700: '#44403c',                800: '#292524',                900: '#1c1917',                950: '#0c0a09',            }        },        fontFamily: {            'sans': ['Inter', 'system-ui', 'sans-serif'],        },        animation: {            'float': 'float 6s ease-in-out infinite',            'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',            'glow': 'glow 2s ease-in-out infinite alternate',            'slide-in': 'slideIn 0.5s ease-out',            'fade-in': 'fadeIn 0.8s ease-out',            'bounce-slow': 'bounce 3s infinite',            'spin-slow': 'spin 8s linear infinite',            'shimmer': 'shimmer 2s infinite',        },        keyframes: {            float: {                '0%, 100%': { transform: 'translateY(0) rotate(0deg)' },                '50%': { transform: 'translateY(-20px) rotate(2deg)' },            },            glow: {                '0%': { boxShadow: '0 0 20px rgba(45, 212, 191, 0.3)' },                '100%': { boxShadow: '0 0 40px rgba(168, 85, 247, 0.5)' },            },            slideIn: {                '0%': { transform: 'translateY(20px)', opacity: '0' },                '100%': { transform: 'translateY(0)', opacity: '1' },            },            fadeIn: {                '0%': { opacity: '0' },                '100%': { opacity: '1' },            },            shimmer: {                '0%': { backgroundPosition: '-200px 0' },                '100%': { backgroundPosition: '200px 0' },            }        },        backgroundImage: {            'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',            'gradient-shimmer': 'linear-gradient(90deg, transparent, rgba(255,255,255,0.1), transparent)',        }    }}\n        }"
            }
            style {
                ".pigeon-bg {background: linear-gradient(135deg, #0c0a09 0%, #1c1917 50%, #0c0a09 100%);position: relative;overflow-x: hidden;\n        }\n        \n        .pigeon-bg::before {content: '';position: absolute;top: 0;left: 0;right: 0;bottom: 0;background-image:     radial-gradient(circle at 10% 20%, rgba(45, 212, 191, 0.08) 0%, transparent 25%),    radial-gradient(circle at 90% 80%, rgba(168, 85, 247, 0.08) 0%, transparent 25%),    radial-gradient(circle at 50% 50%, rgba(45, 212, 191, 0.05) 0%, transparent 50%);pointer-events: none;\n        }\n        \n        .gradient-text {background: linear-gradient(90deg, #2dd4bf 0%, #a855f7 100%);-webkit-background-clip: text;-webkit-text-fill-color: transparent;background-clip: text;position: relative;\n        }\n        \n        .gradient-text::after {content: '';position: absolute;bottom: -2px;left: 0;width: 100%;height: 2px;background: linear-gradient(90deg, #2dd4bf 0%, #a855f7 100%);transform: scaleX(0);transform-origin: left;transition: transform 0.3s ease;\n        }\n        \n        .gradient-text:hover::after {transform: scaleX(1);\n        }\n        \n        .card-hover {transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);position: relative;overflow: hidden;\n        }\n        \n        .card-hover::before {content: '';position: absolute;top: 0;left: -100%;width: 100%;height: 100%;background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.03), transparent);transition: left 0.7s ease;\n        }\n        \n        .card-hover:hover::before {left: 100%;\n        }\n        \n        .card-hover:hover {transform: translateY(-8px) scale(1.02);box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);border-color: rgba(45, 212, 191, 0.3);\n        }\n        \n        .btn-glow {position: relative;overflow: hidden;transition: all 0.3s ease;\n        }\n        \n        .btn-glow::before {content: '';position: absolute;top: -2px;left: -2px;right: -2px;bottom: -2px;background: linear-gradient(45deg, #2dd4bf, #a855f7, #2dd4bf);border-radius: inherit;z-index: -1;opacity: 0;transition: opacity 0.3s ease;\n        }\n        \n        .btn-glow:hover::before {opacity: 1;animation: glow 1.5s ease-in-out infinite alternate;\n        }\n        \n        .btn-glow:hover {transform: translateY(-2px);\n        }\n        \n        .nav-blur {backdrop-filter: blur(12px);-webkit-backdrop-filter: blur(12px);background: rgba(28, 25, 23, 0.85);\n        }\n        \n        .image-hover {transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);\n        }\n        \n        .image-hover:hover {transform: scale(1.05) rotate(1deg);\n        }\n        \n        .scroll-reveal {opacity: 0;transform: translateY(30px);transition: all 0.8s cubic-bezier(0.4, 0, 0.2, 1);\n        }\n        \n        .scroll-reveal.visible {opacity: 1;transform: translateY(0);\n        }\n        \n        .feature-icon {transition: all 0.4s ease;\n        }\n        \n        .card-hover:hover .feature-icon {transform: scale(1.1) rotate(5deg);\n        }\n        \n        @media (max-width: 768px) {.mobile-fade {    animation: fadeIn 0.5s ease-out;}\n        }"
            }
        }
        body { class: "bg-stone-950 text-gray-200 font-sans pigeon-bg min-h-screen",
            nav { class: "sticky top-0 z-50 nav-blur border-b border-stone-800/50 shadow-xl",
                div { class: "container mx-auto px-4 sm:px-6 py-4",
                    div { class: "flex justify-between items-center",
                        div { class: "flex items-center space-x-3",
                            div { class: "w-10 h-10 rounded-full bg-linear-to-r from-teal-500 to-purple-600 flex items-center justify-center animate-glow",
                                i { class: "fas fa-dove text-white text-xl" }
                            }
                            span { class: "text-2xl font-bold",
                                span { class: "text-teal-400", "Pigeon" }
                                span { class: "text-purple-400", "IoT" }
                            }
                        }
                        div { class: "hidden md:flex space-x-8",
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors duration-300 relative group",
                                href: "#",
                                "            Home            "
                                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-teal-400 group-hover:w-full transition-all duration-300" }
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors duration-300 relative group",
                                href: "#",
                                "            About Us            "
                                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-teal-400 group-hover:w-full transition-all duration-300" }
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors duration-300 relative group",
                                href: "#",
                                "            FAQ            "
                                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-teal-400 group-hover:w-full transition-all duration-300" }
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors duration-300 relative group",
                                href: "#",
                                "            Pricing            "
                                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-teal-400 group-hover:w-full transition-all duration-300" }
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors duration-300 relative group",
                                href: "#",
                                "            GitHub            "
                                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-teal-400 group-hover:w-full transition-all duration-300" }
                            }
                        }
                        div { class: "hidden md:block",
                            a {
                                class: "px-6 py-2.5 bg-linear-to-r from-teal-600 to-purple-600 rounded-lg font-semibold btn-glow shadow-lg",
                                href: "#",
                                "            Get Started        "
                            }
                        }
                        button {
                            class: "md:hidden text-gray-300 hover:text-teal-400 transition-colors p-2 rounded-lg hover:bg-stone-800",
                            id: "mobile-menu-button",
                            i { class: "fas fa-bars text-2xl" }
                        }
                    }
                    div {
                        class: "hidden md:hidden mt-4 pb-4 mobile-fade",
                        id: "mobile-menu",
                        div { class: "flex flex-col space-y-3",
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
                                href: "#",
                                i { class: "fas fa-home mr-3" }
                                "Home        "
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
                                href: "#",
                                i { class: "fas fa-info-circle mr-3" }
                                "About Us        "
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
                                href: "#",
                                i { class: "fas fa-question-circle mr-3" }
                                "FAQ        "
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
                                href: "#",
                                i { class: "fas fa-tag mr-3" }
                                "Pricing        "
                            }
                            a {
                                class: "font-medium hover:text-teal-400 transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
                                href: "#",
                                i { class: "fab fa-github mr-3" }
                                "GitHub        "
                            }
                            a {
                                class: "px-6 py-3 bg-linear-to-r from-teal-600 to-purple-600 rounded-lg font-semibold text-center btn-glow mt-2",
                                href: "#",
                                "            Get Started        "
                            }
                        }
                    }
                }
            }
            section { class: "py-16 md:py-28 overflow-hidden",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "flex flex-col lg:flex-row items-center",
                        div { class: "lg:w-1/2 mb-12 lg:mb-0 scroll-reveal",
                            div { class: "inline-flex items-center px-4 py-2 rounded-full bg-stone-800/80 border border-stone-700 mb-6 animate-slide-in",
                                span { class: "w-2 h-2 rounded-full bg-teal-500 mr-2 animate-pulse" }
                                span { class: "text-sm font-medium", "Open Source • Written in Rust" }
                                span { class: "ml-2 px-2 py-0.5 bg-teal-900/50 rounded text-xs font-bold text-teal-300",
                                    "v2.0"
                                }
                            }
                            h1 { class: "text-4xl md:text-6xl lg:text-7xl font-bold mb-6 leading-tight",
                                "            IoT Security That            "
                                span { class: "gradient-text block mt-2", "Doesn't Compromise" }
                                "            Performance        "
                            }
                            p { class: "text-xl text-gray-400 mb-8 max-w-2xl leading-relaxed",
                                "            An open-source IoT platform built in Rust from front to back.             "
                                span { class: "font-semibold text-teal-300",
                                    "You can be sure you're not leaving any performance on the table or holes in your firewall."
                                }
                            }
                            div { class: "flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-6 mb-12",
                                a {
                                    class: "px-8 py-4 bg-linear-to-r from-teal-600 to-purple-600 rounded-lg font-bold text-lg text-center btn-glow shadow-xl",
                                    href: "#",
                                    i { class: "fas fa-rocket mr-2 animate-bounce-slow" }
                                    " Get Started Free            "
                                }
                                a {
                                    class: "px-8 py-4 bg-stone-800/80 border border-stone-700 rounded-lg font-bold text-lg text-center hover:bg-stone-700/80 transition-all duration-300 hover:border-teal-500/30",
                                    href: "#",
                                    i { class: "fab fa-github mr-2" }
                                    " View on GitHub            "
                                }
                            }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
                                div { class: "flex items-center p-4 rounded-xl bg-stone-800/50 border border-stone-700 hover:border-teal-500/30 transition-all duration-300",
                                    div { class: "w-12 h-12 rounded-full bg-teal-900/50 border border-teal-800 flex items-center justify-center mr-4 feature-icon",
                                        i { class: "fas fa-shield-alt text-teal-400 text-xl" }
                                    }
                                    div {
                                        p { class: "font-bold text-lg", "Security First" }
                                        p { class: "text-sm text-gray-400", "Memory-safe by design" }
                                    }
                                }
                                div { class: "flex items-center p-4 rounded-xl bg-stone-800/50 border border-stone-700 hover:border-purple-500/30 transition-all duration-300",
                                    div { class: "w-12 h-12 rounded-full bg-purple-900/50 border border-purple-800 flex items-center justify-center mr-4 feature-icon",
                                        i { class: "fas fa-bolt text-purple-400 text-xl" }
                                    }
                                    div {
                                        p { class: "font-bold text-lg", "High Performance" }
                                        p { class: "text-sm text-gray-400", "Native speed with Rust" }
                                    }
                                }
                            }
                        }
                        div { class: "lg:w-1/2 flex justify-center relative",
                            div { class: "relative w-full max-w-lg",
                                div { class: "relative rounded-2xl overflow-hidden shadow-2xl image-hover border border-stone-700",
                                    img {
                                        alt: "IoT Technology Network",
                                        class: "w-full h-64 md:h-80 object-cover",
                                        src: "https://images.unsplash.com/photo-1718866033984-c3ddab9af2a0?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3ixlib=rb-4.1.0&q=80&w=1080",
                                    }
                                    div { class: "absolute inset-0 bg-linear-to-t from-stone-950/80 via-transparent to-transparent" }
                                    div { class: "absolute bottom-4 left-4 right-4",
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-3 h-3 rounded-full bg-teal-500 animate-pulse" }
                                            span { class: "text-sm font-medium",
                                                "Live IoT Network Visualization"
                                            }
                                        }
                                    }
                                }
                                div { class: "absolute -top-6 -left-6 w-24 h-24 rounded-2xl bg-teal-900/30 border border-teal-800/50 flex items-center justify-center animate-float shadow-lg",
                                    i { class: "fas fa-server text-teal-400 text-2xl" }
                                }
                                div {
                                    class: "absolute -bottom-6 -right-6 w-24 h-24 rounded-2xl bg-purple-900/30 border border-purple-800/50 flex items-center justify-center animate-float shadow-lg",
                                    style: "animation-delay: 1s;",
                                    i { class: "fas fa-satellite-dish text-purple-400 text-2xl" }
                                }
                                div {
                                    class: "absolute top-1/2 -right-8 w-20 h-20 rounded-2xl bg-stone-800/80 border border-stone-700 flex items-center justify-center shadow-lg animate-float",
                                    style: "animation-delay: 2s;",
                                    i { class: "fas fa-microchip text-gray-300 text-xl" }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "py-20 relative",
                div { class: "absolute inset-0 bg-linear-to-b from-transparent via-stone-900/20 to-transparent" }
                div { class: "container mx-auto px-4 sm:px-6 relative",
                    div { class: "text-center mb-16 scroll-reveal",
                        h2 { class: "text-3xl md:text-4xl lg:text-5xl font-bold mb-4",
                            "Built for Engineers, By Engineers"
                        }
                        p { class: "text-xl text-gray-400 max-w-3xl mx-auto",
                            "        A complete IoT platform with native high availability, designed for open-source enthusiasts and B2B applications.    "
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        div { class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            div { class: "w-16 h-16 rounded-2xl bg-linear-to-br from-teal-900 to-teal-800 flex items-center justify-center mb-6 feature-icon shadow-lg",
                                i { class: "fas fa-code text-3xl text-teal-400" }
                            }
                            h3 { class: "text-2xl font-bold mb-4", "100% Rust" }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            From frontend to backend, everything is written in Rust. Enjoy memory safety, zero-cost abstractions, and fearless concurrency.        "
                            }
                            div { class: "flex items-center text-teal-400 font-medium group cursor-pointer",
                                span { class: "group-hover:translate-x-2 transition-transform duration-300",
                                    "Learn more"
                                }
                                i { class: "fas fa-arrow-right ml-2 group-hover:translate-x-2 transition-transform duration-300" }
                            }
                        }
                        div {
                            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            style: "animation-delay: 0.2s;",
                            div { class: "w-16 h-16 rounded-2xl bg-linear-to-br from-purple-900 to-purple-800 flex items-center justify-center mb-6 feature-icon shadow-lg",
                                i { class: "fas fa-shield-alt text-3xl text-purple-400" }
                            }
                            h3 { class: "text-2xl font-bold mb-4", "Security by Design" }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            Rust's ownership model eliminates entire classes of security vulnerabilities. No more buffer overflows or data races.        "
                            }
                            div { class: "flex items-center text-purple-400 font-medium group cursor-pointer",
                                span { class: "group-hover:translate-x-2 transition-transform duration-300",
                                    "Security whitepaper"
                                }
                                i { class: "fas fa-arrow-right ml-2 group-hover:translate-x-2 transition-transform duration-300" }
                            }
                        }
                        div {
                            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            style: "animation-delay: 0.4s;",
                            div { class: "w-16 h-16 rounded-2xl bg-linear-to-br from-teal-900 to-purple-900 flex items-center justify-center mb-6 feature-icon shadow-lg",
                                i { class: "fas fa-tachometer-alt text-3xl text-teal-300" }
                            }
                            h3 { class: "text-2xl font-bold mb-4", "High Availability" }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            Native high-availability architecture ensures your IoT infrastructure remains operational even during partial failures.        "
                            }
                            div { class: "flex items-center text-teal-300 font-medium group cursor-pointer",
                                span { class: "group-hover:translate-x-2 transition-transform duration-300",
                                    "Architecture docs"
                                }
                                i { class: "fas fa-arrow-right ml-2 group-hover:translate-x-2 transition-transform duration-300" }
                            }
                        }
                    }
                }
            }
            section { class: "py-20 bg-linear-to-b from-stone-900/50 to-stone-950/50",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "flex flex-col lg:flex-row items-center gap-12",
                        div { class: "lg:w-1/2 scroll-reveal",
                            div { class: "inline-flex items-center px-4 py-2 rounded-full bg-stone-800/80 border border-stone-700 mb-6",
                                i { class: "fab fa-rust text-orange-500 mr-2" }
                                span { class: "text-sm font-medium", "Rust-Powered Excellence" }
                            }
                            h2 { class: "text-3xl md:text-4xl font-bold mb-6",
                                span { class: "gradient-text", "Memory Safety" }
                                " Meets Performance        "
                            }
                            p { class: "text-gray-400 mb-8 text-lg leading-relaxed",
                                "            Experience the power of Rust's compile-time guarantees. Our platform leverages Rust's ownership system to eliminate entire classes of bugs and security vulnerabilities while delivering C++-level performance.        "
                            }
                            div { class: "space-y-4",
                                div { class: "flex items-start",
                                    div { class: "w-8 h-8 rounded-full bg-teal-900/50 flex items-center justify-center mr-4 mt-1",
                                        i { class: "fas fa-check text-teal-400" }
                                    }
                                    div {
                                        h4 { class: "font-bold text-lg mb-1", "Zero Runtime Overhead" }
                                        p { class: "text-gray-400",
                                            "No garbage collector, no runtime. Pure performance."
                                        }
                                    }
                                }
                                div { class: "flex items-start",
                                    div { class: "w-8 h-8 rounded-full bg-purple-900/50 flex items-center justify-center mr-4 mt-1",
                                        i { class: "fas fa-check text-purple-400" }
                                    }
                                    div {
                                        h4 { class: "font-bold text-lg mb-1", "Fearless Concurrency" }
                                        p { class: "text-gray-400",
                                            "Write concurrent code without data races."
                                        }
                                    }
                                }
                                div { class: "flex items-start",
                                    div { class: "w-8 h-8 rounded-full bg-teal-900/50 flex items-center justify-center mr-4 mt-1",
                                        i { class: "fas fa-check text-teal-400" }
                                    }
                                    div {
                                        h4 { class: "font-bold text-lg mb-1", "Compile-Time Safety" }
                                        p { class: "text-gray-400",
                                            "Catch bugs before they reach production."
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            class: "lg:w-1/2 scroll-reveal",
                            style: "animation-delay: 0.3s;",
                            div { class: "bg-stone-900/80 border border-stone-700 rounded-2xl overflow-hidden shadow-2xl",
                                div { class: "bg-stone-800 px-6 py-4 border-b border-stone-700 flex items-center",
                                    div { class: "flex space-x-2",
                                        div { class: "w-3 h-3 rounded-full bg-red-500" }
                                        div { class: "w-3 h-3 rounded-full bg-yellow-500" }
                                        div { class: "w-3 h-3 rounded-full bg-green-500" }
                                    }
                                    span { class: "ml-4 text-sm font-medium text-gray-300",
                                        "main.rs"
                                    }
                                }
                                div { class: "p-6 font-mono text-sm",
                                    div { class: "text-teal-400 mb-2",
                                        "// Secure IoT device connection"
                                    }
                                    div { class: "text-purple-400 mb-2",
                                        "use pigeon_iot::DeviceManager;"
                                    }
                                    div { class: "mb-4" }
                                    div { class: "text-blue-400 mb-2", "async fn" }
                                    div { class: "text-yellow-400 mb-2", "connect_device" }
                                    div { class: "text-gray-300 mb-2",
                                        "(device_id: &str) -> Result<Device, Error> {"
                                    }
                                    div { class: "ml-4 text-gray-300 mb-2",
                                        "let mut manager = DeviceManager::new()"
                                    }
                                    div { class: "ml-8 text-gray-300 mb-2",
                                        ".with_security(SecurityLevel::Maximum)"
                                    }
                                    div { class: "ml-8 text-gray-300 mb-2",
                                        ".with_encryption(Encryption::AES256GCM)"
                                    }
                                    div { class: "ml-8 text-gray-300 mb-2", ".await?;" }
                                    div { class: "mb-2" }
                                    div { class: "ml-4 text-gray-300 mb-2",
                                        "// Compile-time safety ensures no data races"
                                    }
                                    div { class: "ml-4 text-blue-400 mb-2", "let" }
                                    div { class: "ml-4 text-gray-300 mb-2",
                                        "device = manager.connect(device_id).await?;"
                                    }
                                    div { class: "ml-4 text-gray-300 mb-2", "Ok(device)" }
                                    div { class: "text-gray-300 mb-2", "}" }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "py-20",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "bg-linear-to-r from-teal-900/30 via-purple-900/20 to-teal-900/30 border border-teal-800/30 rounded-3xl p-8 md:p-12 shadow-2xl scroll-reveal",
                        div { class: "flex flex-col lg:flex-row items-center gap-12",
                            div { class: "lg:w-2/3",
                                div { class: "inline-flex items-center px-4 py-2 rounded-full bg-black/40 border border-teal-800 mb-6",
                                    span { class: "text-teal-400 font-bold mr-2 animate-pulse",
                                        "COMING SOON"
                                    }
                                    span { class: "text-sm", "Exclusive partnership" }
                                }
                                h2 { class: "text-3xl md:text-4xl font-bold mb-6",
                                    "                Wireless, the only US IoT carrier with easy access to Non-IP Data Delivery (NIDD)            "
                                }
                                p { class: "text-xl text-gray-300 mb-8 leading-relaxed",
                                    "                Dirt cheap service and high efficiency! Connect your devices without the overhead of traditional IP stacks.            "
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                                    div { class: "flex items-start p-4 rounded-xl bg-black/20 border border-teal-800/20 hover:border-teal-500/30 transition-all duration-300",
                                        i { class: "fas fa-bolt text-teal-400 text-2xl mt-1 mr-4" }
                                        div {
                                            h4 { class: "font-bold text-lg mb-2", "High Efficiency" }
                                            p { class: "text-gray-400 text-sm",
                                                "Reduced data overhead by up to 80% with NIDD technology."
                                            }
                                        }
                                    }
                                    div { class: "flex items-start p-4 rounded-xl bg-black/20 border border-purple-800/20 hover:border-purple-500/30 transition-all duration-300",
                                        i { class: "fas fa-dollar-sign text-purple-400 text-2xl mt-1 mr-4" }
                                        div {
                                            h4 { class: "font-bold text-lg mb-2", "Cost Effective" }
                                            p { class: "text-gray-400 text-sm",
                                                "Pay only for the data you use with our transparent pricing."
                                            }
                                        }
                                    }
                                    div { class: "flex items-start p-4 rounded-xl bg-black/20 border border-teal-800/20 hover:border-teal-500/30 transition-all duration-300",
                                        i { class: "fas fa-shield-alt text-teal-400 text-2xl mt-1 mr-4" }
                                        div {
                                            h4 { class: "font-bold text-lg mb-2", "Enhanced Security" }
                                            p { class: "text-gray-400 text-sm",
                                                "NIDD provides inherent security benefits over traditional IP delivery."
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "lg:w-1/3 flex justify-center",
                                div { class: "relative",
                                    div { class: "w-48 h-48 rounded-full bg-linear-to-br from-teal-700/20 to-purple-700/20 flex items-center justify-center animate-spin-slow border border-teal-800/30",
                                        div { class: "w-36 h-36 rounded-full bg-linear-to-br from-teal-900/30 to-purple-900/30 flex items-center justify-center",
                                            i { class: "fas fa-wifi text-6xl text-teal-400" }
                                        }
                                    }
                                    div { class: "absolute -bottom-4 -right-4 w-16 h-16 rounded-full bg-linear-to-br from-purple-600 to-purple-700 flex items-center justify-center shadow-lg animate-bounce-slow",
                                        span { class: "text-white font-bold text-xl",
                                            "$"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "py-20",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "text-center mb-16 scroll-reveal",
                        h2 { class: "text-3xl md:text-4xl font-bold mb-4", "Built for the Builders" }
                        p { class: "text-xl text-gray-400 max-w-3xl mx-auto",
                            "        Pigeon IoT is designed for engineers and businesses who demand performance, security, and control.    "
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                        div { class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            div { class: "flex items-center mb-6",
                                div { class: "w-14 h-14 rounded-full bg-linear-to-br from-teal-900/50 to-teal-800/50 flex items-center justify-center mr-4 feature-icon",
                                    i { class: "fab fa-osi text-teal-400 text-2xl" }
                                }
                                h3 { class: "text-2xl font-bold", "Open Source" }
                            }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            Fully transparent codebase with an active community. Contribute, fork, or customize to your needs.        "
                            }
                            ul { class: "space-y-4",
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-500 text-sm" }
                                    }
                                    span { "MIT Licensed" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-500 text-sm" }
                                    }
                                    span { "Active community contributions" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-500 text-sm" }
                                    }
                                    span { "Comprehensive documentation" }
                                }
                            }
                        }
                        div {
                            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            style: "animation-delay: 0.2s;",
                            div { class: "flex items-center mb-6",
                                div { class: "w-14 h-14 rounded-full bg-linear-to-br from-purple-900/50 to-purple-800/50 flex items-center justify-center mr-4 feature-icon",
                                    i { class: "fas fa-user-cog text-purple-400 text-2xl" }
                                }
                                h3 { class: "text-2xl font-bold", "Independent Engineers" }
                            }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            Powerful tools for prototyping and production. From hobby projects to commercial deployments.        "
                            }
                            ul { class: "space-y-4",
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-purple-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-purple-500 text-sm" }
                                    }
                                    span { "Easy local development" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-purple-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-purple-500 text-sm" }
                                    }
                                    span { "Extensive device library" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-purple-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-purple-500 text-sm" }
                                    }
                                    span { "Free tier for personal use" }
                                }
                            }
                        }
                        div {
                            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
                            style: "animation-delay: 0.4s;",
                            div { class: "flex items-center mb-6",
                                div { class: "w-14 h-14 rounded-full bg-linear-to-br from-teal-900/50 to-purple-900/50 flex items-center justify-center mr-4 feature-icon",
                                    i { class: "fas fa-building text-teal-300 text-2xl" }
                                }
                                h3 { class: "text-2xl font-bold", "B2B & Enterprise" }
                            }
                            p { class: "text-gray-400 mb-6 leading-relaxed",
                                "            Scalable, secure, and compliant solutions for industrial IoT, smart cities, and enterprise applications.        "
                            }
                            ul { class: "space-y-4",
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-400 text-sm" }
                                    }
                                    span { "SLA guarantees" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-400 text-sm" }
                                    }
                                    span { "Enterprise support" }
                                }
                                li { class: "flex items-center",
                                    div { class: "w-6 h-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                                        i { class: "fas fa-check text-teal-400 text-sm" }
                                    }
                                    span { "Custom deployment options" }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "py-20",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "bg-stone-800/30 border border-stone-700 rounded-3xl overflow-hidden shadow-2xl scroll-reveal",
                        div { class: "relative h-64 md:h-80 lg:h-96",
                            img {
                                alt: "Server Infrastructure",
                                class: "w-full h-full object-cover",
                                src: "https://images.unsplash.com/photo-1762163516269-3c143e04175c?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w4MDcxMzN8MHwxfHNlYXJjaHwxfHxzZXJ2ZXIlMjBkYXRhJTIwY2VudGVyJTIwdGVjaG5vbG9neSUyMGluZnJhc3RydWN0dXJlfGVufDB8MHx8fDE3NzE4OTEzMzl8MA&ixlib=rb-4.1.0&q=80&w=1080",
                            }
                            div { class: "absolute inset-0 bg-linear-to-t from-stone-950 via-stone-950/50 to-transparent" }
                            div { class: "absolute bottom-0 left-0 right-0 p-8",
                                div { class: "flex flex-col md:flex-row items-center justify-between",
                                    div {
                                        h3 { class: "text-2xl md:text-3xl font-bold mb-2",
                                            "Enterprise-Grade Infrastructure"
                                        }
                                        p { class: "text-gray-300",
                                            "Global data centers with 99.99% uptime guarantee"
                                        }
                                    }
                                    div { class: "mt-4 md:mt-0",
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-3 h-3 rounded-full bg-green-500 animate-pulse" }
                                            span { class: "text-sm font-medium",
                                                "All Systems Operational"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "py-20",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "bg-linear-to-r from-teal-900/30 via-purple-900/20 to-teal-900/30 border border-stone-700 rounded-3xl p-8 md:p-12 text-center shadow-2xl scroll-reveal",
                        div { class: "max-w-3xl mx-auto",
                            h2 { class: "text-3xl md:text-4xl lg:text-5xl font-bold mb-6",
                                "Ready to Build Secure IoT Solutions?"
                            }
                            p { class: "text-xl text-gray-300 mb-10 leading-relaxed",
                                "            Join thousands of engineers and businesses who trust Pigeon IoT for their mission-critical applications.        "
                            }
                            div { class: "flex flex-col sm:flex-row justify-center space-y-4 sm:space-y-0 sm:space-x-6 mb-8",
                                a {
                                    class: "px-10 py-4 bg-linear-to-r from-teal-600 to-purple-600 rounded-xl font-bold text-lg btn-glow shadow-xl text-center",
                                    href: "#",
                                    i { class: "fas fa-play-circle mr-2 animate-pulse-slow" }
                                    " Start Free Trial            "
                                }
                                a {
                                    class: "px-10 py-4 bg-stone-800/80 border border-stone-700 rounded-xl font-bold text-lg hover:bg-stone-700/80 transition-all duration-300 hover:border-teal-500/30 text-center",
                                    href: "#",
                                    i { class: "fas fa-book mr-2" }
                                    " Read Documentation            "
                                }
                            }
                            div { class: "inline-flex items-center px-4 py-2 rounded-full bg-black/30 border border-teal-800/30",
                                i { class: "fas fa-gift text-teal-400 mr-2" }
                                span { class: "text-sm",
                                    "No credit card required. Free tier includes up to 100 devices."
                                }
                            }
                        }
                    }
                }
            }
            footer { class: "bg-stone-900/80 border-t border-stone-800/50 py-12",
                div { class: "container mx-auto px-4 sm:px-6",
                    div { class: "flex flex-col lg:flex-row justify-between items-start gap-12",
                        div { class: "lg:w-2/5",
                            div { class: "flex items-center space-x-3 mb-6",
                                div { class: "w-12 h-12 rounded-full bg-linear-to-r from-teal-500 to-purple-600 flex items-center justify-center animate-glow",
                                    i { class: "fas fa-dove text-white text-2xl" }
                                }
                                span { class: "text-3xl font-bold",
                                    span { class: "text-teal-400", "Pigeon" }
                                    span { class: "text-purple-400", "IoT" }
                                }
                            }
                            p { class: "text-gray-400 mb-8 leading-relaxed",
                                "            An open-source IoT platform built in Rust. Security, performance, and reliability by design. Join our community of builders creating the future of connected devices.        "
                            }
                            div { class: "flex space-x-4",
                                a {
                                    class: "w-10 h-10 rounded-full bg-stone-800 flex items-center justify-center hover:bg-teal-900/50 hover:text-teal-400 transition-all duration-300",
                                    href: "#",
                                    i { class: "fab fa-github text-xl" }
                                }
                                a {
                                    class: "w-10 h-10 rounded-full bg-stone-800 flex items-center justify-center hover:bg-blue-900/50 hover:text-blue-400 transition-all duration-300",
                                    href: "#",
                                    i { class: "fab fa-twitter text-xl" }
                                }
                                a {
                                    class: "w-10 h-10 rounded-full bg-stone-800 flex items-center justify-center hover:bg-purple-900/50 hover:text-purple-400 transition-all duration-300",
                                    href: "#",
                                    i { class: "fab fa-discord text-xl" }
                                }
                                a {
                                    class: "w-10 h-10 rounded-full bg-stone-800 flex items-center justify-center hover:bg-red-900/50 hover:text-red-400 transition-all duration-300",
                                    href: "#",
                                    i { class: "fab fa-youtube text-xl" }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 md:grid-cols-3 gap-8 lg:w-3/5",
                            div {
                                h4 { class: "font-bold text-lg mb-4 text-teal-300",
                                    "Product"
                                }
                                ul { class: "space-y-3",
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Features                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Pricing                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Documentation                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    API Reference                "
                                        }
                                    }
                                }
                            }
                            div {
                                h4 { class: "font-bold text-lg mb-4 text-purple-300",
                                    "Company"
                                }
                                ul { class: "space-y-3",
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-purple-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    About Us                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-purple-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Blog                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-purple-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Careers                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-purple-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Contact                "
                                        }
                                    }
                                }
                            }
                            div {
                                h4 { class: "font-bold text-lg mb-4 text-teal-300",
                                    "Community"
                                }
                                ul { class: "space-y-3",
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    GitHub                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Discord                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Twitter                "
                                        }
                                    }
                                    li {
                                        a {
                                            class: "text-gray-400 hover:text-teal-400 transition-colors duration-300 flex items-center group",
                                            href: "#",
                                            i { class: "fas fa-chevron-right text-xs mr-2 opacity-0 group-hover:opacity-100 transition-opacity" }
                                            "                    Stack Overflow                "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "mt-12 pt-8 border-t border-stone-800/50 flex flex-col md:flex-row justify-between items-center",
                        p { class: "text-gray-500 mb-4 md:mb-0 text-sm",
                            "        © 2023 Pigeon IoT. All rights reserved. Built with "
                            i { class: "fas fa-heart text-red-500 mx-1" }
                            " and Rust.    "
                        }
                        div { class: "flex items-center space-x-6",
                            a {
                                class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
                                href: "#",
                                "Privacy Policy"
                            }
                            a {
                                class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
                                href: "#",
                                "Terms of Service"
                            }
                            a {
                                class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
                                href: "#",
                                "Cookie Policy"
                            }
                        }
                    }
                }
            }
            script {
                "// Mobile menu toggle\n        document.getElementById('mobile-menu-button').addEventListener('click', function() {const menu = document.getElementById('mobile-menu');menu.classList.toggle('hidden');this.innerHTML = menu.classList.contains('hidden')     ? '<i class=\"fas fa-bars text-2xl\"></i>'     : '<i class=\"fas fa-times text-2xl\"></i>';\n        });\n\n        // Scroll reveal animation\n        const observerOptions = {threshold: 0.1,rootMargin: '0px 0px -50px 0px'\n        };\n\n        const observer = new IntersectionObserver((entries) => {entries.forEach(entry => {    if (entry.isIntersecting) {        entry.target.classList.add('visible');    }});\n        }, observerOptions);\n\n        // Observe all scroll-reveal elements\n        document.querySelectorAll('.scroll-reveal').forEach(el => {observer.observe(el);\n        });\n\n        // Add smooth scroll for anchor links\n        document.querySelectorAll('a[href^=\"#\"]').forEach(anchor => {anchor.addEventListener('click', function (e) {    e.preventDefault();    const targetId = this.getAttribute('href');    if (targetId === '#') return;        const targetElement = document.querySelector(targetId);    if (targetElement) {        window.scrollTo({            top: targetElement.offsetTop - 80,            behavior: 'smooth'        });    }});\n        });\n\n        // Add hover effect to gradient text\n        document.querySelectorAll('.gradient-text').forEach(el => {el.addEventListener('mouseenter', () => {    el.style.background = 'linear-gradient(90deg, #a855f7 0%, #2dd4bf 100%)';    el.style.webkitBackgroundClip = 'text';    el.style.webkitTextFillColor = 'transparent';});el.addEventListener('mouseleave', () => {    el.style.background = 'linear-gradient(90deg, #2dd4bf 0%, #a855f7 100%)';    el.style.webkitBackgroundClip = 'text';    el.style.webkitTextFillColor = 'transparent';});\n        });\n\n        // Add parallax effect to background\n        window.addEventListener('scroll', () => {const scrolled = window.pageYOffset;const parallaxElements = document.querySelectorAll('.pigeon-bg::before');// Update background position for parallax effectdocument.querySelector('.pigeon-bg').style.backgroundPositionY = `${scrolled * 0.5}px`;\n        });\n\n        // Add loading animation\n        window.addEventListener('load', () => {document.body.classList.add('loaded');// Animate stats countersconst stats = document.querySelectorAll('.stat-number');stats.forEach(stat => {    const target = parseInt(stat.textContent);    let current = 0;    const increment = target / 50;        const timer = setInterval(() => {        current += increment;        if (current >= target) {            current = target;            clearInterval(timer);        }        stat.textContent = Math.floor(current);    }, 30);});\n        });\n\n        // Add keyboard navigation for accessibility\n        document.addEventListener('keydown', (e) => {if (e.key === 'Escape') {    document.getElementById('mobile-menu').classList.add('hidden');    document.getElementById('mobile-menu-button').innerHTML = '<i class=\"fas fa-bars text-2xl\"></i>';}\n        });"
            }
            style {
                "body.loaded .scroll-reveal {animation: slideIn 0.8s ease-out forwards;\n        }\n        \n        @keyframes slideIn {from {    opacity: 0;    transform: translateY(30px);}to {    opacity: 1;    transform: translateY(0);}\n        }\n        \n        /* Accessibility improvements */\n        .focus-visible {outline: 2px solid #2dd4bf;outline-offset: 2px;\n        }\n        \n        /* Print styles */\n        @media print {.btn-glow, .card-hover, .nav-blur {    box-shadow: none !important;    background: white !important;    color: black !important;    border: 1px solid #ccc !important;}.gradient-text {    background: none !important;    -webkit-text-fill-color: black !important;    color: black !important;}\n        }\n        \n        /* Reduced motion preferences */\n        @media (prefers-reduced-motion: reduce) {* {    animation-duration: 0.01ms !important;    animation-iteration-count: 1 !important;    transition-duration: 0.01ms !important;}\n        }"
            }
        }
    }
  }
}
