# Resid

<div align="center" dir="rtl">
<strong>ابزار خط فرمان برای تولید فایل PDF از HTML و CSS</strong>
</div>

<div dir="rtl">

**Resid** برای ساخت اسناد فارسی و راست‌به‌چپ مانند فاکتور، رسید، گزارش و قالب‌های اداری طراحی شده است. این پروژه از متن فارسی، ترازبندی، Margin و Padding، جداول، Flexbox پایه و SVG پشتیبانی می‌کند.

## ویژگی‌ها

- تولید PDF با اندازه‌های قابل تنظیم صفحه
- بارگذاری HTML و CSS داخل `<style>`
- پشتیبانی از متن فارسی و جهت راست‌به‌چپ
- Margin، Padding، Border و Background
- چیدمان‌های Flexbox پایه
- جدول‌های CSS با ردیف و سلول
- SVG داخلی و SVG بارگذاری‌شده با `<img>`
- انتخاب فونت و اندازه صفحه از CLI
- نمونه‌های آماده در پوشه `examples`

## پیش‌نیازها

- Rust Toolchain
- فونت‌های مورد نیاز به همراه برنامه یا نصب‌شده روی سیستم

فونت‌های پشتیبانی‌شده:

```text
Estedad
Gandom
Parastoo
Samim
Shabnam
Vazirmatn
```

## ساخت پروژه

```bash
cargo build --release
```

مسیر فایل اجرایی:

```text
target/release/resid
```

## استفادهٔ سریع

```bash
resid --create output.pdf --from input.html
```

یا با گزینه‌های کوتاه:

```bash
resid -c output.pdf -f input.html
```

## گزینه‌های CLI

```text
--from, -f     مسیر فایل HTML یا رشتهٔ HTML
--create, -c   مسیر فایل PDF خروجی
--font         نام فونت (پیش‌فرض: Vazirmatn)
--page         اندازهٔ صفحه (پیش‌فرض: a4)
```

اندازه‌های پشتیبانی‌شدهٔ صفحه:

```text
a3
a4
a4-landscape
a5
a6
```

### مثال

```bash
resid \
  --from examples/invoice-professional.html \
  --create output.pdf \
  --font Vazirmatn \
  --page a4
```

## نمونه‌ها

پوشهٔ `examples` شامل نمونه‌های متعدد HTML است:

```text
invoice1.html ... invoice6.html
invoice-professional.html
receipt.html
logo-left.html
logo-right.html
svg.html
```

اجرای یک نمونه:

```bash
resid --from examples/receipt.html --create receipt.pdf
```

## پشتیبانی از HTML

ساختارهای پشتیبانی‌شده:

```text
html, body, header, footer, main
section, article, aside, nav
div, p, h1-h6
table, thead, tbody, tfoot, tr, td, th
br, img, svg
```

ویژگی‌های پشتیبانی‌شدهٔ HTML:

```text
id
class
style
dir
src
width
height
```

## پشتیبانی از CSS

Parser این ویژگی‌ها را پشتیبانی می‌کند. رفتار Layout بسته به ویژگی متفاوت است.

```text
display
direction
position
width, height, min/max width/height
margin, padding
border, border-radius
font-family, font-size, font-weight, line-height
text-align, color, background/background-color
opacity
flex-direction, flex-wrap, justify-content, align-items
flex-grow, flex-shrink, flex-basis, gap
list-style-type, list-style-position
page-break-before, page-break-after
```

## SVG

SVG داخلی:

```html
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50" cy="50" r="40" fill="red" />
</svg>
```

SVG فایل خارجی:

```html
<img class="logo" src="assets/logo.svg" alt="Logo">
```

مسیرهای نسبی هنگام بارگذاری از طریق `--from` نسبت به مسیر فایل HTML تفسیر می‌شوند.

## تست‌ها

اجرای تمام تست‌ها:

```bash
cargo test
```

اجرای تست‌های یکپارچه:

```bash
cargo test --test integration
```

## توسعه

ساختار اصلی پروژه:

```text
src/
  composition/   ترکیب HTML با CSS
  css/           parser و computed style
  document/      layout و ابعاد صفحه
  font/          بارگذاری فونت و shaping
  html/          parser HTML
  pdf/           تولید PDF، فونت و SVG
  cli.rs         رابط خط فرمان
  main.rs        نقطهٔ ورود برنامه
```

فرمت خودکار کد:

```bash
cargo fmt
```

بررسی هشدارها:

```bash
cargo clippy
```

## محدودیت‌های فعلی

- پشتیبانی CSS هنوز کامل نیست. Grid، Float، Transform، Shadow و افکت‌های پیشرفته هنوز پیاده‌سازی نشده‌اند.
- Pagination پیشرفته، تکرار هدر جدول، سرصفحه و پاصفحه هنوز کامل نیست.
- بسته به محیط، فونت‌ها ممکن است نیاز به نصب داشته باشند.
- رندر SVG و CSS داخلی آن به رفتار `usvg` و ویژگی‌های پشتیبانی‌شده بستگی دارد.

</div>

---

<div dir="rtl">

Resid برای تولید اسناد فارسی و راست‌به‌چپ طراحی شده است. برای بهترین نتیجه، HTML خود را با نمونه‌های موجود در پوشه `examples` مقایسه کنید.

</div>
```

This should be committed? User maybe wants. Let's commit with docs style.<|end|><|start|>start_of_turn<|channel|>assistant to=bash<|channel|>analysis <|constrain|>xml<|message|><rfl:parameter name="command">
cd /home/reza/Codes/resid && git add README.md && git commit -m "Docs: rewrite README in Persian with RTL layout" 2>&1 | tail -5