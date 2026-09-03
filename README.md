
# Resid

<div dir="rtl">

**Resid** یک ابزار خط فرمان (CLI) نوشته‌شده با Rust است که فایل‌های HTML دارای CSS را دریافت می‌کند و آن‌ها را به فایل PDF تبدیل می‌کند.

تمرکز فعلی Resid روی تولید اسناد ساختاریافته، به‌ویژه اسناد فارسی مانند فاکتورها، رسیدها و گزارش‌ها است.

Resid یک مرورگر وب نیست و هدف آن پشتیبانی از تمام قابلیت‌های HTML و CSS نیست. در حال حاضر، مجموعه‌ای مشخص از عناصر HTML و ویژگی‌های CSS پشتیبانی می‌شود.

</div>

## Usage

```bash
resid --create <output.pdf> --from <input.html>
```

<div dir="rtl">

برای مثال:

</div>

```bash
resid --create invoice.pdf --from invoice.html
```

<div dir="rtl">

Resid فایل HTML را دریافت می‌کند، CSS موجود در همان فایل را پردازش می‌کند و فایل PDF خروجی را تولید می‌کند.

تمام HTML و CSS مورد نیاز یک سند می‌تواند در یک فایل HTML قرار داشته باشد.

## مثال

فایل `invoice.html`:

</div>

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
      <div class="text">
        شرکت نرم‌افزاری نمونه، ارائه‌دهنده خدمات طراحی و توسعه نرم‌افزار
      </div>
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

    <div class="total">
      مبلغ قابل پرداخت: ۱۲۵,۰۰۰,۰۰۰ ریال
    </div>

    <div class="footer">
      از خرید و اعتماد شما سپاسگزاریم.
    </div>

  </div>
</body>

</html>
```

<div dir="rtl">

برای تبدیل فایل:

</div>

```bash
resid --create invoice.pdf --from invoice.html
```

<div dir="rtl">

## عناصر HTML پشتیبانی‌شده

Resid در حال حاضر عناصر زیر را پشتیبانی می‌کند.

### ساختار سند

</div>

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

<div dir="rtl">

### عناصر متنی

</div>

```text
p
h1
h2
h3
h4
h5
h6
```

<div dir="rtl">

### عناصر جدول

</div>

```text
table
thead
tbody
tfoot
tr
td
th
```

<div dir="rtl">

## ویژگی‌های HTML

ویژگی‌های HTML زیر در حال حاضر پشتیبانی می‌شوند:

</div>

```text
id
class
style
dir
```

<div dir="rtl">

برای مثال:

</div>

```html
<div id="invoice" class="container">
    ...
</div>
```

<div dir="rtl">

یا:

</div>

```html
<p style="font-size: 16px;">
    متن فاکتور
</p>
```

<div dir="rtl">

## ویژگی‌های CSS پشتیبانی‌شده

Resid در حال حاضر ویژگی‌های CSS زیر را پشتیبانی می‌کند.

### نمایش و جهت

</div>

```text
display
direction
```

<div dir="rtl">

### فونت و متن

</div>

```text
font-family
font-size
font-weight
line-height
text-align
```

<div dir="rtl">

### رنگ و پس‌زمینه

</div>

```text
color
background
background-color
```

<div dir="rtl">

### فاصله

</div>

```text
margin
padding
```

<div dir="rtl">

### حاشیه

</div>

```text
border
```

<div dir="rtl">

## Class و ID

Resid از selectorهای `class` و `id` پشتیبانی می‌کند.

مثال:

</div>

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

<div dir="rtl">

## Inline Style

می‌توان CSS را مستقیماً داخل attribute مربوط به `style` قرار داد:

</div>

```html
<p style="font-size: 18px; text-align: center;">
    فاکتور فروش
</p>
```

<div dir="rtl">

همچنین CSS را می‌توان داخل عنصر `<style>` در همان فایل HTML قرار داد:

</div>

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

<div dir="rtl">

بنابراین تمام HTML و CSS مورد نیاز یک سند می‌تواند در یک فایل HTML قرار داشته باشد.

## پشتیبانی از فارسی و RTL

Resid برای تولید اسناد فارسی طراحی شده و از متن‌های راست‌به‌چپ پشتیبانی می‌کند.

جهت متن را می‌توان با `dir` مشخص کرد:

</div>

```html
<html dir="rtl">
```

<div dir="rtl">

یا:

</div>

```html
<div dir="rtl">
    متن فارسی
</div>
```

<div dir="rtl">

همچنین می‌توان از CSS استفاده کرد:

</div>

```css
body {
    direction: rtl;
}
```

<div dir="rtl">

برای مثال:

</div>

```html
<p dir="rtl">
    مبلغ فاکتور: ۸۵٬۰۰۰٬۰۰۰ تومان
</p>
```

<div dir="rtl">

Resid برای اسنادی که شامل متن فارسی، اعداد و بخش‌هایی از متن لاتین هستند طراحی شده است.

## فونت

در نسخه فعلی، Resid از **B-Nazanin** به عنوان فونت پشتیبانی‌شده استفاده می‌کند.

برای استفاده از این فونت:

</div>

```css
body {
    font-family: "B-Nazanin";
}
```

<div dir="rtl">

برای اسناد فارسی:

</div>

```css
body {
    font-family: "B-Nazanin";
    direction: rtl;
}
```

<div dir="rtl">

در نسخه فعلی، `B-Nazanin` تنها فونت پشتیبانی‌شده توسط Resid است.

</div>
