# Resid

**Resid** یک ابزار خط فرمان (CLI) نوشته‌شده با Rust است که فایل‌های HTML شامل CSS را دریافت کرده و آن‌ها را به فایل PDF تبدیل می‌کند.

تمرکز فعلی Resid روی تولید اسناد ساختاریافته، به‌خصوص اسناد فارسی مانند فاکتورها، رسیدها و گزارش‌ها است.

Resid یک مرورگر وب نیست و هدف آن پشتیبانی از تمام قابلیت‌های HTML و CSS نیست. در حال حاضر تنها مجموعه‌ای مشخص از عناصر HTML و ویژگی‌های CSS پشتیبانی می‌شود.

## استفاده

ساختار کلی دستور:

```bash
resid --create <input.pdf> --from <output.html>
```

برای مثال:

```bash
resid --create example.pdf --from invoice.html
```

Resid فایل HTML را دریافت کرده، CSS موجود در همان فایل را پردازش می‌کند و فایل PDF خروجی را تولید می‌کند.

تمام HTML و CSS مورد نیاز یک سند می‌تواند در یک فایل قرار داشته باشد.

## مثال


```html
<!DOCTYPE html>
<html lang="fa" dir="rtl">

<head>
  <style>
    body {
      direction: rtl;
      font-family: "B-Nazanin";
      font-size: 10pt;
      line-height: 1.2;
    }

    .invoice {
      margin: 1pt;
      border: 1pt solid #546E7A;
    }

    .header {
      margin: 1pt;
      padding: 1pt;
      text-align: center;
      background: #B0BEC5;
      border: 1pt solid #546E7A;
    }

    .title {
      margin: 1pt;
      padding: 1pt;
      font-size: 22pt;
      text-align: center;
    }

    .subtitle {
      margin: 1pt;
      font-size: 11pt;
      text-align: center;
    }

    .section {
      margin: 1pt;
      padding: 1pt;
      background: #EEEEEE;
      border: 1pt solid #546E7A;
    }

    .section-title {
      margin: 1pt;
      padding: 1pt;
      font-size: 15pt;
      text-align: right;
    }

    .text {
      margin: 1pt;
      padding: 10pt;
      font-size: 12pt;
      line-height: 1.7;
      text-align: right;
    }

    .total {
      margin: 1pt;
      padding: 1pt;
      background: #4DB6AC;
      border: 1pt solid #C8E6C9;
      font-size: 15pt;
      text-align: center;
    }

    .footer {
      text-align: center;
      padding: 1pt;
      font-size: 10pt;
    }
  </style>
</head>

<body>
  <div class="invoice">
    <div class="header">
      <div class="title">فاکتور فروش</div>
      <div class="subtitle">شماره فاکتور: ۱۴۰۵-۰۰۱۲۵</div>
      <div class="subtitle">تاریخ: ۱۴۰۵/۰۶/۱۲</div>
    </div>

    <div class="section">
      <div class="section-title">مشخصات فروشنده</div>
      <div class="text">شرکت نرم‌افزاری نمونه، ارائه‌دهنده خدمات طراحی و توسعه نرم‌افزار</div>
      <div class="text">شماره تماس: ۰۲۱-۱۲۳۴۵۶۷۸</div>
      <div class="text">آدرس: تهران، خیابان نمونه، ساختمان شماره ۱۰</div>
    </div>

    <div class="section">
      <div class="section-title">مشخصات مشتری</div>
      <div class="text">نام مشتری: علی رضایی</div>
      <div class="text">شماره تماس: ۰۹۱۲۱۲۳۴۵۶۷</div>
      <div class="text">آدرس: تهران، خیابان آزادی، کوچه دهم</div>
    </div>

    <div class="section">
      <div class="section-title">شرح خدمات</div>
      <div class="text">
        طراحی و پیاده‌سازی سامانه مدیریت سفارش‌ها، توسعه رابط برنامه‌نویسی کاربردی، ایجاد
        سیستم احراز هویت و تهیه گزارش‌های مدیریتی. این متن عمداً طولانی است تا قابلیت شکست
        خطوط، اندازه‌گیری صحیح متن فارسی، فاصله‌گذاری عمودی و قرارگیری راست‌به‌چپ در چند خط
        مختلف صفحه آزمایش شود.
      </div>
      <div class="text">
        خدمات شامل تحلیل نیازمندی‌ها، طراحی معماری نرم‌افزار، پیاده‌سازی سرویس‌های اصلی،
        آزمایش عملکرد و آماده‌سازی نسخه نهایی برای استفاده در محیط عملیاتی است.
      </div>
    </div>

    <div class="section">
      <div class="section-title">شرایط پرداخت</div>
      <div class="text">
        مبلغ کل فاکتور پس از تأیید نهایی مشتری قابل پرداخت است و کلیه خدمات درج‌شده در این
        فاکتور مطابق توافق طرفین ارائه خواهد شد.
      </div>
    </div>

    <div class="total">مبلغ قابل پرداخت: ۱۲۵,۰۰۰,۰۰۰ ریال</div>

    <div class="footer">از خرید و اعتماد شما سپاسگزاریم.</div>
  </div>
</body>

</html>

```

سپس:

```bash
resid --create invoice.pdf --from invoice.html
```

## HTML پشتیبانی‌شده

Resid در حال حاضر عناصر HTML زیر را پشتیبانی می‌کند:

### ساختار سند

```text
html
body
header
footer
main
section
article
aside
nav
div
```

### متن

```text
p
h1
h2
h3
h4
h5
h6
```

### جدول

```text
table
thead
tbody
tfoot
tr
td
th
```

### Attributes

در حال حاضر attributeهای زیر پشتیبانی می‌شوند:

```text
id
class
style
dir
```

برای مثال:

```html
<div id="invoice" class="container">
    ...
</div>
```

یا:

```html
<p style="font-size: 16px;">
    متن فاکتور
</p>
```

## CSS پشتیبانی‌شده

مجموعه CSS فعلی Resid محدود به ویژگی‌های زیر است.

### Display و Direction

```css
display
direction
```

### فونت و متن

```css
font-family
font-size
font-weight
line-height
text-align
```

### رنگ و پس‌زمینه

```css
color
background
background-color
```

### فاصله

```css
margin
padding
```

### Border

```css
border
```

## نمونه CSS

```css
body {
    direction: rtl;
    margin: 40px;
}

h1 {
    text-align: center;
}

.total {
    font-weight: bold;
    background-color: #eeeeee;
    padding: 10px;
    border: 1px solid #000000;
}
```

## Class و ID

Resid از selectorهای `class` و `id` پشتیبانی می‌کند.

مثال:

```html
<div class="invoice">
    <p id="total">مبلغ کل</p>
</div>
```

```css
.invoice {
    padding: 20px;
}

#total {
    font-weight: bold;
}
```

## Inline Style

امکان استفاده از CSS به صورت inline نیز وجود دارد:

```html
<p style="font-size: 18px; text-align: center;">
    فاکتور فروش
</p>
```

همچنین CSS را می‌توان داخل عنصر `<style>` در همان فایل HTML قرار داد:

```html
<style>
    body {
        margin: 40px;
    }

    h1 {
        text-align: center;
    }
</style>
```

## پشتیبانی از فارسی و RTL

Resid برای تولید اسناد فارسی طراحی شده و از متن‌های راست‌به‌چپ پشتیبانی می‌کند.

جهت متن را می‌توان با `dir` مشخص کرد:

```html
<html dir="rtl">
```

یا:

```html
<div dir="rtl">
    متن فارسی
</div>
```

همچنین می‌توان از CSS استفاده کرد:

```css
body {
    direction: rtl;
}
```

Resid می‌تواند اسنادی را که شامل متن فارسی، اعداد و بخش‌هایی از متن لاتین هستند پردازش کند.

برای مثال:

```html
<p dir="rtl">
    مبلغ فاکتور: ۸۵٬۰۰۰٬۰۰۰ تومان
</p>
```

## فونت

در نسخه فعلی، Resid از **B Nazanin** به عنوان فونت پشتیبانی‌شده استفاده می‌کند.

مثال:

```css
body {
    font-family: "B Nazanin";
}
```

برای اسناد فارسی:

```css
body {
    font-family: "B Nazanin";
    direction: rtl;
}
```
پشتیبانی از فونت‌های بیشتر در نسخه‌های آینده امکان‌پذیر خواهد بود.
