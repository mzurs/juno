<!--
  @component
  Generates an SVG area shape using the `area` function from [d3-shape](https://github.com/d3/d3-shape).
 -->
<script lang="ts">
	import { getContext } from 'svelte';

	// @ts-ignore
	const { data, xGet, yGet, xScale, yScale, extents } = getContext('LayerCake');

	/**  @type {String} [fill='#ab00d610'] The shape's fill color. This is technically optional because it comes with a default value but you'll likely want to replace it with your own color. */
	export let fill = 'rgba(var(--color-primary-rgb), 0.1)';

	$: path =
		'M' +
		$data
			.map((d: number) => {
				return $xGet(d) + ',' + $yGet(d);
			})
			.join('L');

	let area: string;

	$: {
		const yRange = $yScale.range();
		area =
			path +
			('L' +
				$xScale($extents.x ? $extents.x[1] : 0) +
				',' +
				yRange[0] +
				'L' +
				$xScale($extents.x ? $extents.x[0] : 0) +
				',' +
				yRange[0] +
				'Z');
	}
</script>

<path class="path-area" d={area} {fill} />
