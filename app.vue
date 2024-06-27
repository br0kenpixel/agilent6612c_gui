<template>
    <div data-bs-theme="dark" class="bg-dark noselect">
        <NuxtRouteAnnouncer />
        <NuxtPage class="container noselect" />
    </div>
</template>

<style>
p,
h5,
h1 {
    cursor: default;
    color: white;
}

.noselect {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

.force-selectable {
    -webkit-touch-callout: initial !important;
    -webkit-user-select: initial !important;
    -khtml-user-select: initial !important;
    -moz-user-select: initial !important;
    -ms-user-select: initial !important;
    user-select: initial !important;
}
</style>