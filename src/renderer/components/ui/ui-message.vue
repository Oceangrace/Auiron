<script setup lang="ts">
import { inject } from 'vue';
import { localizationContextKey } from '../../localization/localization-context-key';
import { useI18n } from 'vue-i18n';
import { UserMessage } from '../../../core/localization/user-message';
defineProps<{ text: string }>();

const { locale } = useI18n();

const resources = inject(localizationContextKey);

if (resources === undefined) {
  throw new Error('Localization resources are missing');
}

const message = new UserMessage(resources.messages);
</script>
<template>{{ message.translate(text, locale) }}</template>
